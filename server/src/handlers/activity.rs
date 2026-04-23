use axum::{extract::{State, Path, Query}, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::Activity;

#[derive(Deserialize)]
pub struct CreateActivityRequest {
    pub region_id: i64,
    pub name: String,
    pub registration_start_at: String,
    pub registration_end_at: String,
    pub voting_start_at: String,
    pub voting_end_at: String,
    pub max_winner_count: i32,
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

    let region_exists = sqlx::query("SELECT id FROM region WHERE id = ? AND status = 'active'")
        .bind(req.region_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if region_exists.is_none() {
        return Err(AppError::BadRequest("Region not found or inactive".into()));
    }

    let result = sqlx::query(
        "INSERT INTO activity (region_id, name, registration_start_at, registration_end_at, voting_start_at, voting_end_at, max_winner_count, status) VALUES (?, ?, ?, ?, ?, ?, ?, 'draft')"
    )
    .bind(req.region_id)
    .bind(&req.name)
    .bind(&req.registration_start_at)
    .bind(&req.registration_end_at)
    .bind(&req.voting_start_at)
    .bind(&req.voting_end_at)
    .bind(req.max_winner_count)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(CreateActivityResponse {
        id: result.last_insert_id() as i64,
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

    let mut query_str = "SELECT * FROM activity WHERE 1=1".to_string();
    let mut count_str = "SELECT COUNT(*) as total FROM activity WHERE 1=1".to_string();

    if params.status.is_some() {
        query_str.push_str(" AND status = ?");
        count_str.push_str(" AND status = ?");
    }
    if params.region_id.is_some() {
        query_str.push_str(" AND region_id = ?");
        count_str.push_str(" AND region_id = ?");
    }

    query_str.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    let mut q = sqlx::query_as::<_, Activity>(&query_str);
    let mut cq = sqlx::query_scalar::<_, i64>(&count_str);

    if let Some(ref status) = params.status {
        q = q.bind(status);
        cq = cq.bind(status);
    }
    if let Some(ref region_id) = params.region_id {
        q = q.bind(*region_id);
        cq = cq.bind(*region_id);
    }
    q = q.bind(page_size).bind(offset);

    let total = cq.fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    let list = q.fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(ActivityListResponse { list, total }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Activity>, AppError> {
    let activity = sqlx::query_as::<_, Activity>("SELECT * FROM activity WHERE id = ?")
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

    let activity = sqlx::query_as::<_, Activity>("SELECT * FROM activity WHERE id = ?")
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

    sqlx::query("UPDATE activity SET status = ? WHERE id = ?")
        .bind(&req.new_status)
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({ "id": id, "status": req.new_status })))
}

fn validate_status_transition(from: &str, to: &str) -> bool {
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
