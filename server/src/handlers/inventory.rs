use axum::{extract::{State, Path, Query}, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::InventoryItem;
use crate::services::notification::NotificationService;

#[derive(Deserialize)]
pub struct InventoryQuery {
    pub category: Option<String>,
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

pub async fn get_by_store(
    State(state): State<AppState>,
    Path(store_id): Path<i64>,
    Query(params): Query<InventoryQuery>,
) -> Result<Json<StoreInventoryResponse>, AppError> {
    let store_exists = sqlx::query("SELECT id FROM store WHERE id = ?")
        .bind(store_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if store_exists.is_none() {
        return Err(AppError::NotFound("Store not found".into()));
    }

    let mut query_str = "SELECT * FROM inventory_item WHERE store_id = ?".to_string();
    if params.category.is_some() {
        query_str.push_str(" AND category = ?");
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
