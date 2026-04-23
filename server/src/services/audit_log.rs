use sqlx::MySqlPool;

pub struct AuditLogService;

impl AuditLogService {
    #[allow(dead_code)]
    pub async fn log(
        _operator_id: i64,
        _action: &str,
        _target_type: &str,
        _target_id: i64,
        _detail: &str,
    ) {
    }

    pub async fn log_with_pool(
        pool: &MySqlPool,
        operator_id: i64,
        action: &str,
        target_type: &str,
        target_id: i64,
        detail: &str,
    ) {
        let _ = sqlx::query(
            "INSERT INTO audit_log (operator_id, action, target_type, target_id, detail) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(operator_id)
        .bind(action)
        .bind(target_type)
        .bind(target_id)
        .bind(detail)
        .execute(pool)
        .await;
    }
}
