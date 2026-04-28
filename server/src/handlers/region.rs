use axum::{extract::{State, Path, Query}, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::Region;

#[derive(Deserialize)]
pub struct CreateRegionRequest {
    pub name: String,
    pub province: String,
    pub city: String,
    pub coverage_radius_km: Option<i32>,
    pub center_lat: f64,
    pub center_lng: f64,
}

#[derive(Deserialize)]
pub struct ListRegionQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct RegionListResponse {
    pub list: Vec<Region>,
    pub total: i64,
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateRegionRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.name.is_empty() {
        return Err(AppError::BadRequest("Region name is required".into()));
    }

    let radius = req.coverage_radius_km.unwrap_or(10);
    let result = sqlx::query(
        "INSERT INTO region (name, province, city, coverage_radius_km, center_lat, center_lng, status) VALUES (?, ?, ?, ?, ?, ?, 'active')"
    )
    .bind(&req.name)
    .bind(&req.province)
    .bind(&req.city)
    .bind(radius)
    .bind(req.center_lat)
    .bind(req.center_lng)
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
    Query(params): Query<ListRegionQuery>,
) -> Result<Json<RegionListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let mut count_str = "SELECT COUNT(*) as total FROM region WHERE 1=1".to_string();
    let mut query_str = "SELECT * FROM region WHERE 1=1".to_string();

    if params.status.is_some() {
        count_str.push_str(" AND status = ?");
        query_str.push_str(" AND status = ?");
    }
    query_str.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    let mut cq = sqlx::query_scalar::<_, i64>(&count_str);
    let mut q = sqlx::query_as::<_, Region>(&query_str);

    if let Some(ref status) = params.status {
        cq = cq.bind(status);
        q = q.bind(status);
    }
    q = q.bind(page_size).bind(offset);

    let total = cq.fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    let list = q.fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(RegionListResponse { list, total }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Region>, AppError> {
    let region = sqlx::query_as::<_, Region>("SELECT * FROM region WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Region not found".into()))?;

    Ok(Json(region))
}

pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateRegionStatusRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.new_status != "active" && req.new_status != "inactive" {
        return Err(AppError::BadRequest("Invalid status, must be active or inactive".into()));
    }

    let exists = sqlx::query("SELECT id FROM region WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if exists.is_none() {
        return Err(AppError::NotFound("Region not found".into()));
    }

    sqlx::query("UPDATE region SET status = ? WHERE id = ?")
        .bind(&req.new_status)
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({ "id": id, "status": req.new_status })))
}

#[derive(Deserialize)]
pub struct UpdateRegionStatusRequest {
    pub new_status: String,
}
