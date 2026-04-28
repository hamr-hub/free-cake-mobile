use axum::{extract::{State, Path}, Json};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::services::audit_log::AuditLogService;

pub async fn complete_task(
    State(state): State<AppState>,
    Path(task_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let task = sqlx::query("SELECT id, task_status, order_id FROM production_task WHERE id = ?")
        .bind(task_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = task.ok_or_else(|| AppError::NotFound("Production task not found".into()))?;
    let task_status: String = row.get("task_status");
    let order_id: i64 = row.get("order_id");

    if task_status != "in_progress" {
        return Err(AppError::BadRequest("Task must be in_progress to complete".into()));
    }

    sqlx::query("UPDATE production_task SET task_status = 'completed', completed_at = NOW() WHERE id = ?")
        .bind(task_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("UPDATE reward_order SET production_status = 'completed' WHERE id = ?")
        .bind(order_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, 0, "production_task_completed", "production_task", task_id, &format!("order_id={}", order_id)).await;

    Ok(Json(serde_json::json!({ "task_id": task_id, "task_status": "completed", "order_id": order_id })))
}
