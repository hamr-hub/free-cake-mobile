use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::services::audit_log::AuditLogService;

#[derive(Deserialize)]
pub struct VerifyRedeemRequest {
    pub redeem_code: String,
    #[allow(dead_code)]
    pub phone: String,
    pub store_id: i64,
}

#[derive(Serialize)]
pub struct VerifyRedeemResponse {
    pub success: bool,
    pub order_id: i64,
    pub fail_reason: Option<String>,
}

pub async fn verify(
    State(state): State<AppState>,
    Json(req): Json<VerifyRedeemRequest>,
) -> Result<Json<VerifyRedeemResponse>, AppError> {
    if req.redeem_code.is_empty() {
        return Err(AppError::BadRequest("Redeem code is required".into()));
    }

    let lock_key = format!("redeem_lock:{}", req.redeem_code);
    let mut conn = state.redis_client.get_multiplexed_async_connection().await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let locked: i64 = redis::cmd("SETNX")
        .arg(&lock_key)
        .arg("1")
        .arg("EX")
        .arg(30)
        .query_async::<i64>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if locked == 0 {
        return Err(AppError::Conflict("Redeem code is being processed, please retry".into()));
    }

    let code_row = sqlx::query("SELECT id, order_id, code, expires_at, status FROM redeem_code WHERE code = ?")
        .bind(&req.redeem_code)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let result = match code_row {
        None => VerifyRedeemResponse {
            success: false,
            order_id: 0,
            fail_reason: Some("Invalid redeem code".into()),
        },
        Some(row) => {
            let code_id: i64 = row.get("id");
            let order_id: i64 = row.get("order_id");
            let code_status: String = row.get("status");
            let expires_at: chrono::NaiveDateTime = row.get("expires_at");

            if code_status == "used" {
                let existing_record = sqlx::query("SELECT order_id FROM redeem_record WHERE redeem_code_id = ?")
                    .bind(code_id)
                    .fetch_optional(&state.db_pool)
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;
                let existing_order_id = existing_record.map(|r| r.get::<i64, _>("order_id")).unwrap_or(order_id);
                VerifyRedeemResponse {
                    success: true,
                    order_id: existing_order_id,
                    fail_reason: None,
                }
            } else if expires_at < chrono::Utc::now().naive_utc() {
                VerifyRedeemResponse {
                    success: false,
                    order_id,
                    fail_reason: Some("Redeem code has expired".into()),
                }
            } else {
                sqlx::query("UPDATE redeem_code SET status = 'used' WHERE id = ?")
                    .bind(code_id)
                    .execute(&state.db_pool)
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;

                sqlx::query("UPDATE reward_order SET redeem_status = 'redeemed' WHERE id = ?")
                    .bind(order_id)
                    .execute(&state.db_pool)
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;

                sqlx::query(
                    "INSERT INTO redeem_record (order_id, redeem_code_id, store_id, verifier_staff_id, redeem_result) VALUES (?, ?, ?, 0, 'success')"
                )
                .bind(order_id)
                .bind(code_id)
                .bind(req.store_id)
                .execute(&state.db_pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                AuditLogService::log_with_pool(&state.db_pool, 0, "redeem_success", "reward_order", order_id, &format!("code={}, store_id={}", req.redeem_code, req.store_id)).await;

                VerifyRedeemResponse {
                    success: true,
                    order_id,
                    fail_reason: None,
                }
            }
        }
    };

    redis::cmd("DEL")
        .arg(&lock_key)
        .query_async::<()>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(result))
}
