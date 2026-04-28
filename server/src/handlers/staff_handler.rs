use axum::{extract::{State, Path, Query}, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::Staff;

#[derive(Deserialize)]
pub struct CreateStaffRequest {
    pub store_id: i64,
    pub name: String,
    pub phone: String,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct ListStaffQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub store_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct StaffListResponse {
    pub list: Vec<Staff>,
    pub total: i64,
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateStaffRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.name.is_empty() || req.phone.is_empty() {
        return Err(AppError::BadRequest("Name and phone are required".into()));
    }

    let role = req.role.unwrap_or_else(|| "operator".to_string());
    let result = sqlx::query(
        "INSERT INTO staff (store_id, name, phone, role, status) VALUES (?, ?, ?, ?, 'active')"
    )
    .bind(req.store_id)
    .bind(&req.name)
    .bind(&req.phone)
    .bind(&role)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": result.last_insert_id(),
        "status": "active"
    })))
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListStaffQuery>,
) -> Result<Json<StaffListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let mut count_str = "SELECT COUNT(*) as total FROM staff WHERE 1=1".to_string();
    let mut query_str = "SELECT * FROM staff WHERE 1=1".to_string();

    if params.store_id.is_some() {
        count_str.push_str(" AND store_id = ?");
        query_str.push_str(" AND store_id = ?");
    }
    if params.status.is_some() {
        count_str.push_str(" AND status = ?");
        query_str.push_str(" AND status = ?");
    }
    query_str.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    let mut cq = sqlx::query_scalar::<_, i64>(&count_str);
    let mut q = sqlx::query_as::<_, Staff>(&query_str);

    if let Some(ref store_id) = params.store_id {
        cq = cq.bind(*store_id);
        q = q.bind(*store_id);
    }
    if let Some(ref status) = params.status {
        cq = cq.bind(status);
        q = q.bind(status);
    }
    q = q.bind(page_size).bind(offset);

    let total = cq.fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    let list = q.fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(StaffListResponse { list, total }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Staff>, AppError> {
    let staff = sqlx::query_as::<_, Staff>("SELECT * FROM staff WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Staff not found".into()))?;

    Ok(Json(staff))
}

pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateStaffStatusRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.new_status != "active" && req.new_status != "inactive" {
        return Err(AppError::BadRequest("Invalid status".into()));
    }

    let exists = sqlx::query("SELECT id FROM staff WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if exists.is_none() {
        return Err(AppError::NotFound("Staff not found".into()));
    }

    sqlx::query("UPDATE staff SET status = ? WHERE id = ?")
        .bind(&req.new_status)
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({ "id": id, "status": req.new_status })))
}

#[derive(Deserialize)]
pub struct UpdateStaffStatusRequest {
    pub new_status: String,
}
