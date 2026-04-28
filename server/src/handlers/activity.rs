use axum::{extract::{State, Path, Query}, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::Activity;
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
pub struct ActivityListResponse {
    pub list: Vec<Activity>,
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
    if req.name.is_empty() {
        return Err(AppError::BadRequest("Activity name is required".into()));
    }
    if req.max_winner_count <= 0 {
        return Err(AppError::BadRequest("max_winner_count must be positive".into()));
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
    let cream_type = req.cream_type.unwrap_or_else(|| "animal".to_string());
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
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM activity WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM activity WHERE 1=1");

    if let Some(ref status) = params.status {
        query_builder.push(" AND status = ");
        query_builder.push_bind(status);
        count_builder.push(" AND status = ");
        count_builder.push_bind(status);
    }
    if let Some(region_id) = params.region_id {
        query_builder.push(" AND region_id = ");
        query_builder.push_bind(region_id);
        count_builder.push(" AND region_id = ");
        count_builder.push_bind(region_id);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<Activity>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(ActivityListResponse { list, total }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Activity>, AppError> {
    let activity = sqlx::query_as::<_, Activity>("SELECT * FROM activity WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Activity not found".into()))?;

    Ok(Json(activity))
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
