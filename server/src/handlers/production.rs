use axum::{extract::{State, Path, Extension, Json}};
use serde::Deserialize;
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::services::audit_log::AuditLogService;
use crate::app_middleware::auth::Claims;

pub async fn start_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let task = sqlx::query("SELECT id, task_status FROM production_task WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = task.ok_or_else(|| AppError::NotFound("Production task not found".into()))?;
    let task_status: String = row.get("task_status");

    validate_transition(&task_status, "in_progress")
        .map_err(AppError::BadRequest)?;

    sqlx::query("UPDATE production_task SET task_status = 'in_progress', started_at = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(task_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "production_task_started", "production_task", task_id, "pending → in_progress").await;

    Ok(Json(serde_json::json!({ "task_id": task_id, "task_status": "in_progress" })))
}

pub async fn pause_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let task = sqlx::query("SELECT id, task_status FROM production_task WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = task.ok_or_else(|| AppError::NotFound("Production task not found".into()))?;
    let task_status: String = row.get("task_status");

    validate_transition(&task_status, "paused")
        .map_err(AppError::BadRequest)?;

    sqlx::query("UPDATE production_task SET task_status = 'paused', paused_at = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(task_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "production_task_paused", "production_task", task_id, "in_progress → paused").await;

    Ok(Json(serde_json::json!({ "task_id": task_id, "task_status": "paused" })))
}

pub async fn resume_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let task = sqlx::query("SELECT id, task_status FROM production_task WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = task.ok_or_else(|| AppError::NotFound("Production task not found".into()))?;
    let task_status: String = row.get("task_status");

    validate_transition(&task_status, "in_progress")
        .map_err(AppError::BadRequest)?;

    sqlx::query("UPDATE production_task SET task_status = 'in_progress', resumed_at = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(task_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "production_task_resumed", "production_task", task_id, &format!("{} → in_progress", task_status)).await;

    Ok(Json(serde_json::json!({ "task_id": task_id, "task_status": "in_progress" })))
}

#[derive(Deserialize)]
pub struct ErrorReportRequest {
    pub error_description: String,
}

pub async fn report_error(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<i64>,
    Json(req): Json<ErrorReportRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.error_description.trim().is_empty() {
        return Err(AppError::BadRequest("Error description is required".into()));
    }

    let task = sqlx::query("SELECT id, task_status FROM production_task WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = task.ok_or_else(|| AppError::NotFound("Production task not found".into()))?;
    let task_status: String = row.get("task_status");

    validate_transition(&task_status, "error")
        .map_err(AppError::BadRequest)?;

    sqlx::query("UPDATE production_task SET task_status = 'error', error_description = $1 WHERE id = $2")
        .bind(&req.error_description)
        .bind(task_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "production_task_error", "production_task", task_id, &format!("{} → error: {}", task_status, req.error_description)).await;

    Ok(Json(serde_json::json!({ "task_id": task_id, "task_status": "error" })))
}

pub async fn cancel_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let task = sqlx::query("SELECT id, task_status, order_id FROM production_task WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = task.ok_or_else(|| AppError::NotFound("Production task not found".into()))?;
    let task_status: String = row.get("task_status");
    let order_id: i64 = row.get("order_id");

    validate_transition(&task_status, "cancelled")
        .map_err(AppError::BadRequest)?;

    sqlx::query("UPDATE production_task SET task_status = 'cancelled', cancelled_at = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(task_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("UPDATE reward_order SET production_status = 'cancelled' WHERE id = $1 AND production_status != 'completed'")
        .bind(order_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "production_task_cancelled", "production_task", task_id, &format!("{} → cancelled, order_id={}", task_status, order_id)).await;

    Ok(Json(serde_json::json!({ "task_id": task_id, "task_status": "cancelled" })))
}

pub async fn complete_task(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let task = sqlx::query("SELECT id, task_status, order_id FROM production_task WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = task.ok_or_else(|| AppError::NotFound("Production task not found".into()))?;
    let task_status: String = row.get("task_status");
    let order_id: i64 = row.get("order_id");

    validate_transition(&task_status, "completed")
        .map_err(AppError::BadRequest)?;

    sqlx::query("UPDATE production_task SET task_status = 'completed', completed_at = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(task_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("UPDATE reward_order SET production_status = 'completed' WHERE id = $1")
        .bind(order_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "production_task_completed", "production_task", task_id, &format!("order_id={}", order_id)).await;

    Ok(Json(serde_json::json!({ "task_id": task_id, "task_status": "completed", "order_id": order_id })))
}

fn validate_transition(current: &str, target: &str) -> Result<(), String> {
    match (current, target) {
        ("pending", "in_progress") => Ok(()),
        ("in_progress", "paused") => Ok(()),
        ("paused", "in_progress") => Ok(()),
        ("in_progress", "completed") => Ok(()),
        ("in_progress" | "paused", "error") => Ok(()),
        ("error", "in_progress") => Ok(()),
        ("pending" | "in_progress" | "paused" | "error", "cancelled") => Ok(()),
        ("completed", "cancelled") => Err("Cannot cancel a completed task".into()),
        _ => Err(format!("Invalid transition: {} → {}", current, target)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_transitions() {
        assert!(validate_transition("pending", "in_progress").is_ok());
        assert!(validate_transition("in_progress", "paused").is_ok());
        assert!(validate_transition("paused", "in_progress").is_ok());
        assert!(validate_transition("error", "in_progress").is_ok());
        assert!(validate_transition("in_progress", "completed").is_ok());
        assert!(validate_transition("in_progress", "error").is_ok());
        assert!(validate_transition("paused", "error").is_ok());
        assert!(validate_transition("pending", "cancelled").is_ok());
        assert!(validate_transition("in_progress", "cancelled").is_ok());
        assert!(validate_transition("paused", "cancelled").is_ok());
        assert!(validate_transition("error", "cancelled").is_ok());
    }

    #[test]
    fn cannot_cancel_completed() {
        assert_eq!(
            validate_transition("completed", "cancelled").unwrap_err(),
            "Cannot cancel a completed task"
        );
    }

    #[test]
    fn invalid_transitions() {
        assert!(validate_transition("pending", "completed").is_err());
        assert!(validate_transition("pending", "paused").is_err());
        assert!(validate_transition("completed", "in_progress").is_err());
        assert!(validate_transition("cancelled", "in_progress").is_err());
        assert!(validate_transition("completed", "paused").is_err());
    }

    #[test]
    fn same_state_not_allowed() {
        assert!(validate_transition("pending", "pending").is_err());
        assert!(validate_transition("in_progress", "in_progress").is_err());
        assert!(validate_transition("error", "error").is_err());
    }

    #[test]
    fn error_description_required() {
        let req = ErrorReportRequest { error_description: "   ".to_string() };
        assert!(req.error_description.trim().is_empty());
    }
}
