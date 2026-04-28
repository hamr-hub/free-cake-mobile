use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::Row;
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
    let row = sqlx::query(
        r#"
        SELECT
            (SELECT COUNT(*) FROM region WHERE status = 'active') AS active_regions,
            (SELECT COUNT(*) FROM contest_entry WHERE created_at >= CURRENT_DATE) AS today_entries,
            (SELECT COUNT(*) FROM vote_record WHERE created_at >= CURRENT_DATE) AS today_votes,
            (SELECT COUNT(*) FROM vote_record WHERE created_at >= CURRENT_DATE AND vote_status IN ('frozen', 'invalid')) AS risk_votes,
            (SELECT COUNT(*) FROM reward_order WHERE production_status = 'pending') AS pending_production,
            (SELECT COUNT(*) FROM redeem_record WHERE redeem_at >= CURRENT_DATE) AS today_redeemed,
            (SELECT COUNT(*) FROM reward_order WHERE redeem_status IN ('pending', 'redeemed')) AS today_total_orders,
            (SELECT COUNT(DISTINCT store_id) FROM inventory_item WHERE quantity <= safety_threshold) AS low_inventory_stores
        "#
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let today_votes: i64 = row.get("today_votes");
    let risk_votes: i64 = row.get("risk_votes");
    let risk_vote_ratio = if today_votes > 0 {
        risk_votes as f64 / today_votes as f64
    } else {
        0.0
    };

    let today_redeemed: i64 = row.get("today_redeemed");
    let today_total_orders: i64 = row.get("today_total_orders");
    let today_redeem_rate = if today_total_orders > 0 {
        today_redeemed as f64 / today_total_orders as f64
    } else {
        0.0
    };

    Ok(Json(DashboardStats {
        active_regions: row.get("active_regions"),
        today_entries: row.get("today_entries"),
        today_votes,
        risk_vote_ratio,
        pending_production: row.get("pending_production"),
        today_redeem_rate,
        low_inventory_stores: row.get("low_inventory_stores"),
    }))
}
