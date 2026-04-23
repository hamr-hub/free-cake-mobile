use axum::{extract::{State, Path}, Json};
use serde::{Deserialize, Serialize};
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

    let region_exists = sqlx::query("SELECT id FROM region WHERE id = ? AND status = 'active'")
        .bind(req.region_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if region_exists.is_none() {
        return Err(AppError::BadRequest("Region not found or inactive".into()));
    }

    let result = sqlx::query(
        "INSERT INTO store (region_id, name, address, lat, lng, daily_capacity, contact_name, contact_phone) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(req.region_id)
    .bind(&req.name)
    .bind(&req.address)
    .bind(req.lat)
    .bind(req.lng)
    .bind(req.daily_capacity)
    .bind(&req.contact_name)
    .bind(&req.contact_phone)
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
) -> Result<Json<StoreListResponse>, AppError> {
    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM store")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = sqlx::query_as::<_, Store>("SELECT * FROM store ORDER BY created_at DESC")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(StoreListResponse { list, total }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Store>, AppError> {
    let store = sqlx::query_as::<_, Store>("SELECT * FROM store WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Store not found".into()))?;

    Ok(Json(store))
}
