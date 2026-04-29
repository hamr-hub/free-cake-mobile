use axum::{extract::{State, Path, Extension, Query}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::services::audit_log::AuditLogService;
use crate::app_middleware::auth::Claims;
use crate::services::validation;

#[derive(Deserialize)]
pub struct PriceListQuery {
    pub region_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct PriceListResponse {
    pub list: Vec<PriceConfigItem>,
    pub total: i64,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct PriceConfigItem {
    pub id: i64,
    pub region_id: i64,
    pub cake_size: String,
    pub cream_type: String,
    pub price: f64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<PriceListQuery>,
) -> Result<Json<PriceListResponse>, AppError> {
    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM price_config WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM price_config WHERE 1=1");

    if let Some(rid) = params.region_id {
        query_builder.push(" AND region_id = "); query_builder.push_bind(rid);
        count_builder.push(" AND region_id = "); count_builder.push_bind(rid);
    }
    if let Some(ref s) = params.status {
        query_builder.push(" AND status = "); query_builder.push_bind(s);
        count_builder.push(" AND status = "); count_builder.push_bind(s);
    }

    query_builder.push(" ORDER BY region_id, cake_size, cream_type");

    let total: i64 = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let rows = query_builder.build().fetch_all(&state.db_pool).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list: Vec<PriceConfigItem> = rows.iter().map(|r| PriceConfigItem {
        id: r.get::<i64, _>("id"),
        region_id: r.get::<i64, _>("region_id"),
        cake_size: r.get::<String, _>("cake_size"),
        cream_type: r.get::<String, _>("cream_type"),
        price: r.get::<f64, _>("price"),
        status: r.get::<String, _>("status"),
        created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
        updated_at: r.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
    }).collect();

    Ok(Json(PriceListResponse { list, total }))
}

#[derive(Deserialize)]
pub struct CreatePriceRequest {
    pub region_id: i64,
    pub cake_size: String,
    pub cream_type: String,
    pub price: f64,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreatePriceRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.price < 0.0 {
        return Err(AppError::BadRequest("Price cannot be negative".into()));
    }
    validation::validate_cake_size(&req.cake_size)?;
    validation::validate_cream_type(&req.cream_type)?;
    let row = sqlx::query(
        "INSERT INTO price_config (region_id, cake_size, cream_type, price) VALUES ($1, $2, $3, $4) RETURNING id"
    )
    .bind(req.region_id)
    .bind(&req.cake_size)
    .bind(&req.cream_type)
    .bind(req.price)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let id: i64 = row.get("id");
    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "create", "price_config", id,
        &format!("region_id={}, {} {}/{}", req.region_id, req.cake_size, req.cream_type, req.price)).await;

    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)]
pub struct UpdatePriceRequest {
    pub price: Option<f64>,
    pub status: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePriceRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Some(p) = req.price {
        if p < 0.0 { return Err(AppError::BadRequest("Price cannot be negative".into())); }
    }

    let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE price_config SET id = id");
    if let Some(p) = req.price { builder.push(", price = "); builder.push_bind(p); }
    if let Some(ref s) = req.status { builder.push(", status = "); builder.push_bind(s); }
    builder.push(" WHERE id = "); builder.push_bind(id);

    builder.build().execute(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "update", "price_config", id, "updated price config").await;

    Ok(Json(serde_json::json!({ "id": id })))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<PriceConfigItem>, AppError> {
    let row = sqlx::query("SELECT * FROM price_config WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Price config not found".into()))?;

    Ok(Json(PriceConfigItem {
        id: row.get::<i64, _>("id"),
        region_id: row.get::<i64, _>("region_id"),
        cake_size: row.get::<String, _>("cake_size"),
        cream_type: row.get::<String, _>("cream_type"),
        price: row.get::<f64, _>("price"),
        status: row.get::<String, _>("status"),
        created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
        updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
    }))
}
