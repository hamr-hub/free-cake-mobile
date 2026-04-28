use axum::{extract::{State, Path, Query}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::Store;

#[derive(Deserialize)]
pub struct CreateStoreRequest {
    pub region_id: i64,
    pub name: String,
    pub address: String,
    pub lat: f64,
    pub lng: f64,
    pub daily_capacity: i32,
    pub contact_name: String,
    pub contact_phone: String,
}

#[derive(Deserialize)]
pub struct ListStoreQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub region_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct StoreListResponse {
    pub list: Vec<Store>,
    pub total: i64,
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateStoreRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.name.is_empty() {
        return Err(AppError::BadRequest("Store name is required".into()));
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
        "INSERT INTO store (region_id, name, address, lat, lng, daily_capacity, contact_name, contact_phone) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id"
    )
    .bind(req.region_id)
    .bind(&req.name)
    .bind(&req.address)
    .bind(req.lat)
    .bind(req.lng)
    .bind(req.daily_capacity)
    .bind(&req.contact_name)
    .bind(&req.contact_phone)
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
    Query(params): Query<ListStoreQuery>,
) -> Result<Json<StoreListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM store WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM store WHERE 1=1");

    if let Some(region_id) = params.region_id {
        query_builder.push(" AND region_id = ");
        query_builder.push_bind(region_id);
        count_builder.push(" AND region_id = ");
        count_builder.push_bind(region_id);
    }
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

    let list = query_builder.build_query_as::<Store>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(StoreListResponse { list, total }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Store>, AppError> {
    let store = sqlx::query_as::<_, Store>("SELECT * FROM store WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Store not found".into()))?;

    Ok(Json(store))
}
