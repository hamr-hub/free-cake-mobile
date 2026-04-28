use axum::{extract::{State, Path, Query}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
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
    let row = sqlx::query(
        "INSERT INTO region (name, province, city, coverage_radius_km, center_lat, center_lng, status) VALUES ($1, $2, $3, $4, $5, $6, 'active') RETURNING id"
    )
    .bind(&req.name)
    .bind(&req.province)
    .bind(&req.city)
    .bind(radius)
    .bind(req.center_lat)
    .bind(req.center_lng)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": row.get::<i64, _>("id"),
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

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM region WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM region WHERE 1=1");

    if let Some(ref status) = params.status {
        query_builder.push(" AND status = ");
        query_builder.push_bind(status);
        count_builder.push(" AND status = ");
        count_builder.push_bind(status);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<Region>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(RegionListResponse { list, total }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Region>, AppError> {
    let region = sqlx::query_as::<_, Region>("SELECT * FROM region WHERE id = $1")
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

    let exists = sqlx::query("SELECT id FROM region WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if exists.is_none() {
        return Err(AppError::NotFound("Region not found".into()));
    }

    sqlx::query("UPDATE region SET status = $1 WHERE id = $2")
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
