use axum::{extract::{State, Path, Query, Extension}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::Store;
use crate::app_middleware::auth::Claims;
use crate::services::validation;

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
    validation::validate_string_max(&req.name, 100, "Store name")?;
    validation::validate_string_max(&req.address, 500, "Address")?;
    validation::validate_phone(&req.contact_phone)?;
    if req.daily_capacity < 0 {
        return Err(AppError::BadRequest("daily_capacity cannot be negative".into()));
    }
    if !(-90.0..=90.0).contains(&req.lat) || !(-180.0..=180.0).contains(&req.lng) {
        return Err(AppError::BadRequest("Invalid lat/lng range".into()));
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

#[derive(Deserialize)]
pub struct UpdateStoreRequest {
    pub name: Option<String>,
    pub region_id: Option<i64>,
    pub address: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub daily_capacity: Option<i32>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<crate::app_middleware::auth::Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateStoreRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE store SET updated_at = NOW()");
    if let Some(ref v) = req.name { builder.push(", name = "); builder.push_bind(v); }
    if let Some(v) = req.region_id { builder.push(", region_id = "); builder.push_bind(v); }
    if let Some(ref v) = req.address { builder.push(", address = "); builder.push_bind(v); }
    if let Some(v) = req.lat { builder.push(", lat = "); builder.push_bind(v); }
    if let Some(v) = req.lng { builder.push(", lng = "); builder.push_bind(v); }
    if let Some(v) = req.daily_capacity { builder.push(", daily_capacity = "); builder.push_bind(v); }
    if let Some(ref v) = req.contact_name { builder.push(", contact_name = "); builder.push_bind(v); }
    if let Some(ref v) = req.contact_phone { builder.push(", contact_phone = "); builder.push_bind(v); }
    builder.push(" WHERE id = "); builder.push_bind(id);

    builder.build().execute(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    crate::services::audit_log::AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "update", "store", id, "updated store fields").await;

    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)]
pub struct UpdateStoreStatusRequest {
    pub status: String,
}

pub async fn update_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateStoreStatusRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.status != "active" && req.status != "inactive" {
        return Err(AppError::BadRequest("Status must be active or inactive".into()));
    }
    sqlx::query("UPDATE store SET status = $1 WHERE id = $2")
        .bind(&req.status)
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    crate::services::audit_log::AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "store_status_update", "store", id, &format!("status={}", req.status)).await;

    Ok(Json(serde_json::json!({ "id": id, "status": req.status })))
}
