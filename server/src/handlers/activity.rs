use axum::{extract::{State, Path, Query, Extension}, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::Activity;
use crate::services::audit_log::AuditLogService;
use crate::app_middleware::auth::Claims;
use crate::services::validation;
use sqlx::Row;

#[derive(Deserialize)]
pub struct CreateActivityRequest {
    pub region_id: i64,
    pub name: String,
    pub registration_start_at: String,
    pub registration_end_at: String,
    pub voting_start_at: String,
    pub voting_end_at: String,
    pub max_winner_count: i32,
    pub max_votes_per_day: Option<i32>,
    pub cake_size: Option<String>,
    pub cream_type: Option<String>,
    pub ai_generation_rate_limit: Option<i32>,
    pub allow_ai_entry: Option<bool>,
    pub weighted_voting: Option<bool>,
}

#[derive(Deserialize)]
pub struct ListActivityQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub status: Option<String>,
    pub region_id: Option<i64>,
}

#[derive(Serialize)]
pub struct ActivityListItem {
    #[serde(flatten)]
    pub activity: Activity,
    pub region_name: String,
    pub current_entry_count: i64,
    pub current_vote_count: i64,
    pub banner_url: Option<String>,
    pub rules: Option<ActivityRuleResponse>,
}

#[derive(Serialize)]
pub struct ActivityListResponse {
    pub list: Vec<ActivityListItem>,
    pub total: i64,
}

#[derive(Serialize)]
pub struct CreateActivityResponse {
    pub id: i64,
    pub status: String,
}

const VALID_STATUSES: [&str; 7] = [
    "draft", "registration_open", "voting_open",
    "voting_closed", "settled", "redeeming", "finished",
];

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateActivityRequest>,
) -> Result<Json<CreateActivityResponse>, AppError> {
    validation::validate_string_max(&req.name, 200, "Activity name")?;
    if req.max_winner_count <= 0 {
        return Err(AppError::BadRequest("max_winner_count must be positive".into()));
    }
    if let Some(ref cs) = req.cake_size {
        validation::validate_cake_size(cs)?;
    }
    if let Some(ref ct) = req.cream_type {
        validation::validate_cream_type(ct)?;
    }

    let region_exists = sqlx::query("SELECT id FROM region WHERE id = $1 AND status = 'active'")
        .bind(req.region_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if region_exists.is_none() {
        return Err(AppError::BadRequest("Region not found or inactive".into()));
    }

    let row = sqlx::query(
        "INSERT INTO activity (region_id, name, registration_start_at, registration_end_at, voting_start_at, voting_end_at, max_winner_count, status) VALUES ($1, $2, $3, $4, $5, $6, $7, 'draft') RETURNING id"
    )
    .bind(req.region_id)
    .bind(&req.name)
    .bind(&req.registration_start_at)
    .bind(&req.registration_end_at)
    .bind(&req.voting_start_at)
    .bind(&req.voting_end_at)
    .bind(req.max_winner_count)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let activity_id = row.get::<i64, _>("id");

    // Create default activity_rule
    let max_votes = req.max_votes_per_day.unwrap_or(3);
    let cake_size = req.cake_size.unwrap_or_else(|| "6inch".to_string());
    validation::validate_cake_size(&cake_size)?;
    let cream_type = req.cream_type.unwrap_or_else(|| "animal".to_string());
    validation::validate_cream_type(&cream_type)?;
    let ai_rate = req.ai_generation_rate_limit.unwrap_or(5);

    sqlx::query(
        "INSERT INTO activity_rule (activity_id, max_votes_per_day, cake_size, cream_type, decoration_params) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(activity_id)
    .bind(max_votes)
    .bind(&cake_size)
    .bind(&cream_type)
    .bind(format!("{{\"ai_generation_rate_limit\":{}}}", ai_rate))
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(CreateActivityResponse {
        id: activity_id,
        status: "draft".to_string(),
    }))
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListActivityQuery>,
) -> Result<Json<ActivityListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM activity WHERE 1=1");

    if let Some(ref status) = params.status {
        count_builder.push(" AND status = "); count_builder.push_bind(status);
    }
    if let Some(region_id) = params.region_id {
        count_builder.push(" AND region_id = "); count_builder.push_bind(region_id);
    }

    let total = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = String::from(
        r#"SELECT a.*, COALESCE(r.name, '未知赛区') AS region_name,
           (SELECT COUNT(*) FROM contest_entry e WHERE e.activity_id = a.id AND e.status = 'active') AS current_entry_count,
           (SELECT COUNT(*) FROM vote_record v WHERE v.activity_id = a.id AND v.vote_status = 'valid') AS current_vote_count
           FROM activity a LEFT JOIN region r ON a.region_id = r.id WHERE 1=1"#
    );
    if let Some(ref status) = params.status {
        q.push_str(" AND a.status = '");
        q.push_str(status);
        q.push('\'');
    }
    if let Some(region_id) = params.region_id {
        q.push_str(&format!(" AND a.region_id = {}", region_id));
    }
    q.push_str(" ORDER BY a.created_at DESC");

    // Pagination
    let limit: i64 = page_size;
    let off: i64 = offset;
    q.push_str(&format!(" LIMIT {} OFFSET {}", limit, off));

    let rows = sqlx::query(&q)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut list = Vec::with_capacity(rows.len());
    for row in &rows {
        let activity = Activity {
            id: row.get("id"),
            region_id: row.get("region_id"),
            name: row.get("name"),
            registration_start_at: row.get("registration_start_at"),
            registration_end_at: row.get("registration_end_at"),
            voting_start_at: row.get("voting_start_at"),
            voting_end_at: row.get("voting_end_at"),
            max_winner_count: row.get("max_winner_count"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };
        let region_name: String = row.get("region_name");
        let current_entry_count: i64 = row.get("current_entry_count");
        let current_vote_count: i64 = row.get("current_vote_count");
        let banner_url: Option<String> = row.try_get("banner_url").ok();

        let rule_row = sqlx::query(
            "SELECT id, activity_id, max_votes_per_day, COALESCE(cake_size, '6inch') as cake_size, \
             COALESCE(cream_type, 'animal') as cream_type, decoration_params, COALESCE(status, 'active') as status \
             FROM activity_rule WHERE activity_id = $1"
        )
        .bind(activity.id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        let rules = rule_row.map(|r| ActivityRuleResponse {
            id: r.get("id"),
            activity_id: r.get("activity_id"),
            max_votes_per_day: r.get("max_votes_per_day"),
            cake_size: r.get("cake_size"),
            cream_type: r.get("cream_type"),
            decoration_params: r.get("decoration_params"),
            status: r.get("status"),
        });

        list.push(ActivityListItem {
            activity,
            region_name,
            current_entry_count,
            current_vote_count,
            banner_url,
            rules,
        });
    }

    Ok(Json(ActivityListResponse { list, total }))
}

#[derive(Serialize)]
pub struct ActivityDetailResponse {
    #[serde(flatten)]
    pub activity: Activity,
    pub region_name: String,
    pub current_entry_count: i64,
    pub current_vote_count: i64,
    pub rules: Option<ActivityRuleResponse>,
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ActivityDetailResponse>, AppError> {
    let row = sqlx::query(
        r#"SELECT a.*, COALESCE(r.name, '未知赛区') AS region_name,
           (SELECT COUNT(*) FROM contest_entry e WHERE e.activity_id = a.id AND e.status = 'active') AS current_entry_count,
           (SELECT COUNT(*) FROM vote_record v WHERE v.activity_id = a.id AND v.vote_status = 'valid') AS current_vote_count
           FROM activity a LEFT JOIN region r ON a.region_id = r.id WHERE a.id = $1"#
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .ok_or_else(|| AppError::NotFound("Activity not found".into()))?;

    let activity = Activity {
        id: row.get("id"),
        region_id: row.get("region_id"),
        name: row.get("name"),
        registration_start_at: row.get("registration_start_at"),
        registration_end_at: row.get("registration_end_at"),
        voting_start_at: row.get("voting_start_at"),
        voting_end_at: row.get("voting_end_at"),
        max_winner_count: row.get("max_winner_count"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    let region_name: String = row.get("region_name");
    let current_entry_count: i64 = row.get("current_entry_count");
    let current_vote_count: i64 = row.get("current_vote_count");

    let rules = sqlx::query(
        "SELECT id, activity_id, max_votes_per_day, COALESCE(cake_size, '6inch') as cake_size, \
         COALESCE(cream_type, 'animal') as cream_type, decoration_params, \
         COALESCE(status, 'active') as status \
         FROM activity_rule WHERE activity_id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let rules = rules.map(|r| ActivityRuleResponse {
        id: r.get("id"),
        activity_id: r.get("activity_id"),
        max_votes_per_day: r.get("max_votes_per_day"),
        cake_size: r.get("cake_size"),
        cream_type: r.get("cream_type"),
        decoration_params: r.get("decoration_params"),
        status: r.get("status"),
    });

    Ok(Json(ActivityDetailResponse {
        activity,
        region_name,
        current_entry_count,
        current_vote_count,
        rules,
    }))
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub new_status: String,
}

pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !VALID_STATUSES.contains(&req.new_status.as_str()) {
        return Err(AppError::BadRequest(format!("Invalid status: {}", req.new_status)));
    }

    let activity = sqlx::query_as::<_, Activity>("SELECT * FROM activity WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Activity not found".into()))?;

    let valid = validate_status_transition(&activity.status, &req.new_status);
    if !valid {
        return Err(AppError::BadRequest(format!(
            "Cannot transition from {} to {}", activity.status, req.new_status
        )));
    }

    sqlx::query("UPDATE activity SET status = $1 WHERE id = $2")
        .bind(&req.new_status)
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({ "id": id, "status": req.new_status })))
}

pub fn validate_status_transition(from: &str, to: &str) -> bool {
    match from {
        "draft" => to == "registration_open",
        "registration_open" => to == "voting_open" || to == "draft",
        "voting_open" => to == "voting_closed",
        "voting_closed" => to == "settled",
        "settled" => to == "redeeming",
        "redeeming" => to == "finished",
        _ => false,
    }
}

#[derive(Deserialize)]
pub struct UpdateActivityRequest {
    pub name: Option<String>,
    pub max_winner_count: Option<i32>,
    pub registration_start_at: Option<String>,
    pub registration_end_at: Option<String>,
    pub voting_start_at: Option<String>,
    pub voting_end_at: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<crate::app_middleware::auth::Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateActivityRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE activity SET updated_at = NOW()");
    if let Some(ref v) = req.name { builder.push(", name = "); builder.push_bind(v); }
    if let Some(v) = req.max_winner_count { builder.push(", max_winner_count = "); builder.push_bind(v); }
    if let Some(ref v) = req.registration_start_at { builder.push(", registration_start_at = "); builder.push_bind(v); }
    if let Some(ref v) = req.registration_end_at { builder.push(", registration_end_at = "); builder.push_bind(v); }
    if let Some(ref v) = req.voting_start_at { builder.push(", voting_start_at = "); builder.push_bind(v); }
    if let Some(ref v) = req.voting_end_at { builder.push(", voting_end_at = "); builder.push_bind(v); }
    builder.push(" WHERE id = "); builder.push_bind(id);

    builder.build().execute(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    crate::services::audit_log::AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "update", "activity", id, "updated activity fields").await;

    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ActivityRuleResponse {
    pub id: i64,
    pub activity_id: i64,
    pub max_votes_per_day: i32,
    pub cake_size: String,
    pub cream_type: String,
    pub decoration_params: Option<serde_json::Value>,
    pub status: String,
}

pub async fn get_rules(
    State(state): State<AppState>,
    Path(activity_id): Path<i64>,
) -> Result<Json<ActivityRuleResponse>, AppError> {
    let row = sqlx::query(
        "SELECT id, activity_id, max_votes_per_day, COALESCE(cake_size, '6inch') as cake_size, \
         COALESCE(cream_type, 'animal') as cream_type, decoration_params, \
         COALESCE(status, 'active') as status \
         FROM activity_rule WHERE activity_id = $1"
    )
    .bind(activity_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .ok_or_else(|| AppError::NotFound("Activity rules not found".into()))?;

    Ok(Json(ActivityRuleResponse {
        id: row.get("id"),
        activity_id: row.get("activity_id"),
        max_votes_per_day: row.get("max_votes_per_day"),
        cake_size: row.get("cake_size"),
        cream_type: row.get("cream_type"),
        decoration_params: row.get("decoration_params"),
        status: row.get("status"),
    }))
}

#[derive(Deserialize)]
pub struct UpdateRulesRequest {
    pub max_votes_per_day: Option<i32>,
    pub cake_size: Option<String>,
    pub cream_type: Option<String>,
}

pub async fn update_rules(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(activity_id): Path<i64>,
    Json(req): Json<UpdateRulesRequest>,
) -> Result<Json<ActivityRuleResponse>, AppError> {
    let mut builder = sqlx::QueryBuilder::new("UPDATE activity_rule SET ");
    let mut updates = 0;
    if let Some(v) = req.max_votes_per_day {
        if updates > 0 { builder.push(", "); }
        builder.push("max_votes_per_day = "); builder.push_bind(v);
        updates += 1;
    }
    if let Some(ref v) = req.cake_size {
        validation::validate_cake_size(v)?;
        if updates > 0 { builder.push(", "); }
        builder.push("cake_size = "); builder.push_bind(v);
        updates += 1;
    }
    if let Some(ref v) = req.cream_type {
        validation::validate_cream_type(v)?;
        if updates > 0 { builder.push(", "); }
        builder.push("cream_type = "); builder.push_bind(v);
        updates += 1;
    }
    if updates == 0 {
        return Err(AppError::BadRequest("No fields to update".into()));
    }
    builder.push(", updated_at = NOW() WHERE activity_id = "); builder.push_bind(activity_id);
    builder.push(" RETURNING id, activity_id, max_votes_per_day, COALESCE(cake_size, '6inch') as cake_size, COALESCE(cream_type, 'animal') as cream_type, decoration_params, COALESCE(status, 'active') as status");

    let row = builder.build_query_as::<ActivityRuleResponse>()
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Activity rules not found".into()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "update_rules", "activity_rule", row.id, &format!("activity_id={}", activity_id)).await;

    Ok(Json(row))
}
