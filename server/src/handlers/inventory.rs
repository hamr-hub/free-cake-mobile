use axum::{extract::{State, Path, Query, Extension}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::InventoryItem;
use crate::services::notification::NotificationService;
use crate::services::audit_log::AuditLogService;

#[derive(Deserialize)]
pub struct InventoryQuery {
    pub category: Option<String>,
    pub store_id: Option<i64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct InventoryAlert {
    pub item_id: i64,
    pub item_name: String,
    pub current_quantity: f64,
    pub safety_threshold: f64,
}

#[derive(Serialize)]
pub struct StoreInventoryResponse {
    pub items: Vec<InventoryItem>,
    pub alerts: Vec<InventoryAlert>,
}

#[derive(Serialize)]
pub struct InventoryListResponse {
    pub list: Vec<InventoryItem>,
    pub total: i64,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<InventoryQuery>,
) -> Result<Json<InventoryListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM inventory_item WHERE 1=1");
    let mut list_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM inventory_item WHERE 1=1");

    if let Some(sid) = params.store_id {
        count_builder.push(" AND store_id = "); count_builder.push_bind(sid);
        list_builder.push(" AND store_id = "); list_builder.push_bind(sid);
    }
    if let Some(ref cat) = params.category {
        count_builder.push(" AND category = "); count_builder.push_bind(cat);
        list_builder.push(" AND category = "); list_builder.push_bind(cat);
    }

    let total: i64 = count_builder.build_query_scalar()
        .fetch_one(&state.db_pool).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    list_builder.push(" ORDER BY id LIMIT "); list_builder.push_bind(page_size);
    list_builder.push(" OFFSET "); list_builder.push_bind(offset);

    let list = list_builder.build_query_as::<InventoryItem>()
        .fetch_all(&state.db_pool).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(InventoryListResponse { list, total }))
}

pub async fn get_by_store(
    State(state): State<AppState>,
    Path(store_id): Path<i64>,
    Query(params): Query<InventoryQuery>,
) -> Result<Json<StoreInventoryResponse>, AppError> {
    let store_exists = sqlx::query("SELECT id FROM store WHERE id = $1")
        .bind(store_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if store_exists.is_none() {
        return Err(AppError::NotFound("Store not found".into()));
    }

    let mut query_str = "SELECT * FROM inventory_item WHERE store_id = $1".to_string();
    if params.category.is_some() {
        query_str.push_str(" AND category = $2");
    }

    let mut q = sqlx::query_as::<_, InventoryItem>(&query_str).bind(store_id);
    if let Some(ref cat) = params.category {
        q = q.bind(cat);
    }

    let items = q.fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let alerts: Vec<InventoryAlert> = items.iter()
        .filter(|item| item.quantity <= item.safety_threshold)
        .map(|item| InventoryAlert {
            item_id: item.id,
            item_name: item.name.clone(),
            current_quantity: item.quantity,
            safety_threshold: item.safety_threshold,
        })
        .collect();

    for alert in &alerts {
        NotificationService::send_inventory_alert(&state.db_pool, store_id, alert.item_id).await;
    }

    Ok(Json(StoreInventoryResponse { items, alerts }))
}

#[derive(Deserialize)]
pub struct CreateInventoryItemRequest {
    pub store_id: i64,
    pub name: String,
    pub category: String,
    pub unit: String,
    pub quantity: f64,
    pub safety_threshold: f64,
}

pub async fn create_item(
    State(state): State<AppState>,
    Extension(claims): Extension<crate::app_middleware::auth::Claims>,
    Json(req): Json<CreateInventoryItemRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.name.is_empty() {
        return Err(AppError::BadRequest("Item name is required".into()));
    }

    let row = sqlx::query(
        "INSERT INTO inventory_item (store_id, name, category, unit, quantity, safety_threshold) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
    )
    .bind(req.store_id)
    .bind(&req.name)
    .bind(&req.category)
    .bind(&req.unit)
    .bind(req.quantity)
    .bind(req.safety_threshold)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let id: i64 = row.get("id");
    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "create", "inventory_item", id, &format!("name={}", req.name)).await;

    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)]
pub struct CreateInventoryTxnRequest {
    pub store_id: i64,
    pub item_id: i64,
    pub txn_type: String,
    pub quantity: f64,
    pub reason: String,
}

pub async fn create_txn(
    State(state): State<AppState>,
    Extension(claims): Extension<crate::app_middleware::auth::Claims>,
    Json(req): Json<CreateInventoryTxnRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !["replenish", "consume", "damage"].contains(&req.txn_type.as_str()) {
        return Err(AppError::BadRequest("txn_type must be replenish, consume, or damage".into()));
    }

    let delta = match req.txn_type.as_str() {
        "replenish" => req.quantity,
        _ => -req.quantity,
    };

    // Execute both operations in a single DB transaction for atomicity
    let mut tx = state.db_pool.begin().await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let row = sqlx::query(
        "INSERT INTO inventory_txn (store_id, item_id, txn_type, quantity, reason) VALUES ($1, $2, $3, $4, $5) RETURNING id"
    )
    .bind(req.store_id)
    .bind(req.item_id)
    .bind(&req.txn_type)
    .bind(req.quantity)
    .bind(&req.reason)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let txn_id: i64 = row.get("id");

    sqlx::query("UPDATE inventory_item SET quantity = quantity + $1, updated_at = NOW() WHERE id = $2")
        .bind(delta)
        .bind(req.item_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    tx.commit().await
        .map_err(|e| AppError::Internal(format!("Transaction commit failed: {}", e)))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "create_txn", "inventory_txn", txn_id, &format!("item_id={}, type={}, qty={}", req.item_id, req.txn_type, req.quantity)).await;

    Ok(Json(serde_json::json!({ "id": txn_id })))
}

#[derive(Deserialize)]
pub struct UpdateInventoryItemRequest {
    pub quantity: Option<f64>,
    pub safety_threshold: Option<f64>,
}

pub async fn update_item(
    State(state): State<AppState>,
    Extension(claims): Extension<crate::app_middleware::auth::Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateInventoryItemRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE inventory_item SET updated_at = NOW()");
    if let Some(v) = req.quantity { builder.push(", quantity = "); builder.push_bind(v); }
    if let Some(v) = req.safety_threshold { builder.push(", safety_threshold = "); builder.push_bind(v); }
    builder.push(" WHERE id = "); builder.push_bind(id);

    builder.build().execute(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "update", "inventory_item", id, "updated inventory item").await;

    Ok(Json(serde_json::json!({ "id": id })))
}

pub async fn show_item(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<InventoryItem>, AppError> {
    let item = sqlx::query_as::<_, InventoryItem>("SELECT * FROM inventory_item WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Inventory item not found".into()))?;

    Ok(Json(item))
}
