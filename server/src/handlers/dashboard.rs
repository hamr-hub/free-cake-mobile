use axum::{extract::State, Json};
use serde::Serialize;
use crate::AppState;
use crate::errors::AppError;

#[derive(Serialize)]
pub struct DashboardStats {
    pub active_regions: i64,
    pub today_entries: i64,
    pub today_votes: i64,
    pub risk_vote_ratio: f64,
    pub pending_production: i64,
    pub today_redeem_rate: f64,
    pub low_inventory_stores: i64,
}

pub async fn stats(
    State(state): State<AppState>,
) -> Result<Json<DashboardStats>, AppError> {
    let active_regions = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM region WHERE status = 'active'")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let today_entries = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM contest_entry WHERE created_at >= CURDATE()"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let today_votes = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM vote_record WHERE created_at >= CURDATE()"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let risk_votes = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM vote_record WHERE created_at >= CURDATE() AND vote_status IN ('frozen', 'invalid')"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let risk_vote_ratio = if today_votes > 0 {
        risk_votes as f64 / today_votes as f64
    } else {
        0.0
    };

    let pending_production = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM reward_order WHERE production_status = 'pending'"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let today_redeemed = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM redeem_record WHERE redeem_at >= CURDATE()"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let today_total_orders = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM reward_order WHERE redeem_status IN ('pending', 'redeemed')"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let today_redeem_rate = if today_total_orders > 0 {
        today_redeemed as f64 / today_total_orders as f64
    } else {
        0.0
    };

    let low_inventory_stores = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(DISTINCT store_id) FROM inventory_item WHERE quantity <= safety_threshold"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(DashboardStats {
        active_regions,
        today_entries,
        today_votes,
        risk_vote_ratio,
        pending_production,
        today_redeem_rate,
        low_inventory_stores,
    }))
}
