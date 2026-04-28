use axum::{extract::{State, Path}, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::services::audit_log::AuditLogService;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ScheduleRequest {
    pub store_id: i64,
    pub scheduled_date: String,
    pub priority: Option<i32>,
}

#[derive(Serialize)]
pub struct ScheduleResponse {
    pub batch_id: i64,
    pub task_ids: Vec<i64>,
}

pub async fn schedule(
    State(state): State<AppState>,
    Path(order_id): Path<i64>,
    Json(req): Json<ScheduleRequest>,
) -> Result<Json<ScheduleResponse>, AppError> {
    let order = sqlx::query("SELECT id, winner_id, template_id, production_status FROM reward_order WHERE id = ?")
        .bind(order_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = order.ok_or_else(|| AppError::NotFound("Order not found".into()))?;
    let production_status: String = row.get("production_status");
    let template_id: i64 = row.get("template_id");

    if production_status != "pending" {
        return Err(AppError::BadRequest("Order is not in pending status".into()));
    }

    let store = sqlx::query("SELECT daily_capacity, status FROM store WHERE id = ?")
        .bind(req.store_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let store_row = store.ok_or_else(|| AppError::NotFound("Store not found".into()))?;
    let daily_capacity: i32 = store_row.get("daily_capacity");
    let store_status: String = store_row.get("status");
    if store_status != "active" {
        return Err(AppError::BadRequest("Store is not active".into()));
    }

    let scheduled_dt = req.scheduled_date.parse::<NaiveDateTime>()
        .map_err(|_| AppError::BadRequest("Invalid scheduled_date format, expected YYYY-MM-DDTHH:MM:SS".into()))?;

    let existing_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM production_task WHERE store_id = ? AND task_status IN ('pending', 'in_progress') AND created_at >= ?"
    )
    .bind(req.store_id)
    .bind(scheduled_dt)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    if existing_count >= daily_capacity as i64 {
        return Err(AppError::Conflict("Store capacity exceeded for the scheduled date".into()));
    }

    let activity_row = sqlx::query("SELECT activity_id FROM winner_record WHERE id = (SELECT winner_id FROM reward_order WHERE id = ?)")
        .bind(order_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let activity_id: i64 = activity_row.get("activity_id");

    let batch_result = sqlx::query(
        "INSERT INTO production_batch (store_id, activity_id, scheduled_date, total_count, status) VALUES (?, ?, ?, 1, 'pending')"
    )
    .bind(req.store_id)
    .bind(activity_id)
    .bind(scheduled_dt)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let batch_id = batch_result.last_insert_id() as i64;

    let task_result = sqlx::query(
        "INSERT INTO production_task (batch_id, order_id, store_id, template_id, task_status) VALUES (?, ?, ?, ?, 'pending')"
    )
    .bind(batch_id)
    .bind(order_id)
    .bind(req.store_id)
    .bind(template_id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let task_id = task_result.last_insert_id() as i64;

    sqlx::query("UPDATE reward_order SET store_id = ?, scheduled_date = ?, production_status = 'scheduled' WHERE id = ?")
        .bind(req.store_id)
        .bind(scheduled_dt)
        .bind(order_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, 0, "order_scheduled", "reward_order", order_id, &format!("batch_id={}, store_id={}", batch_id, req.store_id)).await;

    Ok(Json(ScheduleResponse {
        batch_id,
        task_ids: vec![task_id],
    }))
}

#[derive(Serialize)]
pub struct ResendCodeResponse {
    pub new_code: String,
    pub order_id: i64,
}

pub async fn resend_code(
    State(state): State<AppState>,
    Path(order_id): Path<i64>,
) -> Result<Json<ResendCodeResponse>, AppError> {
    let order = sqlx::query("SELECT id, redeem_status FROM reward_order WHERE id = ?")
        .bind(order_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let _row = order.ok_or_else(|| AppError::NotFound("Order not found".into()))?;

    let old_codes = sqlx::query("SELECT id, code, status FROM redeem_code WHERE order_id = ?")
        .bind(order_id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    for code_row in &old_codes {
        let code_id: i64 = code_row.get("id");
        let code_status: String = code_row.get("status");
        if code_status == "valid" || code_status == "expired" {
            sqlx::query("UPDATE redeem_code SET status = 'invalid' WHERE id = ?")
                .bind(code_id)
                .execute(&state.db_pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
        }
    }

    let new_code = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let expires_at = "2099-12-31 00:00:00";

    sqlx::query("INSERT INTO redeem_code (order_id, code, expires_at, status) VALUES (?, ?, ?, 'valid')")
        .bind(order_id)
        .bind(&new_code)
        .bind(expires_at)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("UPDATE reward_order SET redeem_status = 'pending' WHERE id = ? AND redeem_status IN ('redeemed', 'expired')")
        .bind(order_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, 0, "resend_redeem_code", "reward_order", order_id, &format!("new_code={}", new_code)).await;

    Ok(Json(ResendCodeResponse { new_code, order_id }))
}
