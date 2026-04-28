use sqlx::PgPool;
use serde_json::json;

pub struct NotificationService;

impl NotificationService {
    pub async fn send_settle_notification(pool: &PgPool, activity_id: i64) {
        let _ = sqlx::query(
            "INSERT INTO audit_log (operator_id, action, target_type, target_id, detail) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(0)
        .bind("notification_sent")
        .bind("activity")
        .bind(activity_id)
        .bind("settle_notification_sent")
        .execute(pool)
        .await;
    }

    pub async fn send_inventory_alert(pool: &PgPool, store_id: i64, item_id: i64) {
        let detail = json!({ "store_id": store_id, "item_id": item_id, "alert_type": "low_stock" }).to_string();
        let _ = sqlx::query(
            "INSERT INTO audit_log (operator_id, action, target_type, target_id, detail) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(0)
        .bind("notification_sent")
        .bind("inventory")
        .bind(item_id)
        .bind(detail.as_str())
        .execute(pool)
        .await;
    }
}
