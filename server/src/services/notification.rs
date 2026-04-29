use sqlx::PgPool;
use sqlx::Row;
use serde_json::json;

pub struct NotificationService;

impl NotificationService {
    pub async fn send_settle_notification(pool: &PgPool, activity_id: i64) {
        let winners = sqlx::query(
            r#"SELECT u.phone, rc.code
               FROM winner_record wr
               JOIN app_user u ON u.id = wr.user_id
               JOIN reward_order ro ON ro.winner_id = wr.id
               JOIN redeem_code rc ON rc.order_id = ro.id
               WHERE wr.activity_id = $1 AND rc.status = 'valid'
               ORDER BY wr.rank ASC"#
        )
        .bind(activity_id)
        .fetch_all(pool)
        .await;

        if let Ok(rows) = winners {
            for row in &rows {
                let phone: String = row.get("phone");
                let code: String = row.get("code");
                tracing::info!(
                    target: "notification",
                    "SETTLE_NOTIFY activity_id={} phone={} redeem_code={} message=恭喜您获奖！请凭核销码到店领取免费蛋糕",
                    activity_id, phone, code
                );
            }
            tracing::info!(
                target: "notification",
                "SETTLE_NOTIFY activity_id={} winners_notified={}",
                activity_id, rows.len()
            );
        }

        let _ = sqlx::query(
            "INSERT INTO audit_log (operator_id, action, target_type, target_id, detail) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(0)
        .bind("notification_sent")
        .bind("activity")
        .bind(activity_id)
        .bind(format!("settle_notification_sent: activity_id={activity_id}"))
        .execute(pool)
        .await;
    }

    pub async fn send_inventory_alert(pool: &PgPool, store_id: i64, item_id: i64) {
        let manager = sqlx::query(
            r#"SELECT s.phone FROM staff s WHERE s.store_id = $1 AND s.role = 'manager' AND s.status = 'active' LIMIT 1"#
        )
        .bind(store_id)
        .fetch_optional(pool)
        .await;

        if let Ok(Some(row)) = manager {
            let phone: String = row.get("phone");
            tracing::info!(
                target: "notification",
                "INVENTORY_ALERT store_id={} item_id={} manager_phone={} message=库存不足，请及时补货",
                store_id, item_id, phone
            );
        }

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
