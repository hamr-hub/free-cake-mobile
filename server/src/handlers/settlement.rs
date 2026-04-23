use axum::{extract::{State, Path}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::services::audit_log::AuditLogService;
use crate::services::notification::NotificationService;

#[derive(Deserialize)]
pub struct SettleRequest {
    pub force: Option<bool>,
}

#[derive(Serialize)]
pub struct SettleResponse {
    pub winner_count: i32,
    pub order_count: i32,
    pub redeem_code_count: i32,
}

pub async fn settle(
    State(state): State<AppState>,
    Path(activity_id): Path<i64>,
    Json(req): Json<SettleRequest>,
) -> Result<Json<SettleResponse>, AppError> {
    let activity = sqlx::query("SELECT status, max_winner_count, voting_end_at, region_id FROM activity WHERE id = ?")
        .bind(activity_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = activity.ok_or_else(|| AppError::NotFound("Activity not found".into()))?;
    let status: String = row.get("status");
    let max_winners: i32 = row.get("max_winner_count");
    let region_id: i64 = row.get("region_id");

    if status != "voting_closed" && !req.force.unwrap_or(false) {
        if status == "settled" {
            return Err(AppError::Conflict("Activity already settled".into()));
        }
        return Err(AppError::BadRequest("Activity is not in voting_closed state".into()));
    }

    if status == "settled" && !req.force.unwrap_or(false) {
        return Err(AppError::Conflict("Activity already settled".into()));
    }

    let entries = sqlx::query(
        "SELECT id, user_id, valid_vote_count FROM contest_entry WHERE activity_id = ? AND status = 'active' ORDER BY valid_vote_count DESC LIMIT ?"
    )
    .bind(activity_id)
    .bind(max_winners)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let winner_count = entries.len() as i32;
    let mut order_count = 0i32;
    let mut redeem_code_count = 0i32;

    for (idx, entry) in entries.iter().enumerate() {
        let entry_id: i64 = entry.get("id");
        let user_id: i64 = entry.get("user_id");
        let valid_votes: i32 = entry.get("valid_vote_count");
        let rank = (idx + 1) as i32;

        sqlx::query(
            "INSERT INTO winner_record (activity_id, entry_id, user_id, rank, valid_vote_count, status) VALUES (?, ?, ?, ?, ?, 'confirmed')"
        )
        .bind(activity_id)
        .bind(entry_id)
        .bind(user_id)
        .bind(rank)
        .bind(valid_votes)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        let winner_id = sqlx::query("SELECT LAST_INSERT_ID() as id")
            .fetch_one(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let winner_id: i64 = winner_id.get("id");

        let stores = sqlx::query("SELECT id FROM store WHERE region_id = ? AND status = 'active' LIMIT 1")
            .bind(region_id)
            .fetch_optional(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let store_id: i64 = stores.map(|s| s.get::<i64, _>("id")).unwrap_or(0);

        let templates = sqlx::query("SELECT selected_template_id FROM contest_entry WHERE id = ?")
            .bind(entry_id)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let template_id: i64 = templates.get("selected_template_id");

        sqlx::query(
            "INSERT INTO reward_order (winner_id, store_id, order_type, template_id, production_status, redeem_status) VALUES (?, ?, 'free', ?, 'pending', 'pending')"
        )
        .bind(winner_id)
        .bind(store_id)
        .bind(template_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        order_count += 1;

        let order_id_result = sqlx::query("SELECT LAST_INSERT_ID() as id")
            .fetch_one(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let order_id: i64 = order_id_result.get("id");

        let code = uuid::Uuid::new_v4().to_string()[..8].to_string();
        let expires_at = "2099-12-31 00:00:00";

        sqlx::query(
            "INSERT INTO redeem_code (order_id, code, expires_at, status) VALUES (?, ?, ?, 'valid')"
        )
        .bind(order_id)
        .bind(&code)
        .bind(expires_at)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        redeem_code_count += 1;
    }

    sqlx::query("UPDATE activity SET status = 'settled' WHERE id = ?")
        .bind(activity_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, 0, "activity_settled", "activity", activity_id, &format!("winner_count={}", winner_count)).await;
    NotificationService::send_settle_notification(activity_id).await;

    Ok(Json(SettleResponse {
        winner_count,
        order_count,
        redeem_code_count,
    }))
}
