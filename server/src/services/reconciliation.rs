use sqlx::{PgPool, Row};
use serde::Serialize;
use crate::errors::AppError;

#[derive(Serialize)]
pub struct ReconciliationResult {
    pub payment_mismatches: Vec<PaymentMismatch>,
    pub vote_count_drifts: Vec<VoteCountDrift>,
    pub inventory_drifts: Vec<InventoryDrift>,
}

#[derive(Serialize)]
pub struct PaymentMismatch {
    pub order_id: i64,
    pub order_pay_status: String,
    pub paid_records: i64,
    pub total_paid_amount: Option<f64>,
}

#[derive(Serialize)]
pub struct VoteCountDrift {
    pub entry_id: i64,
    pub stored_count: i32,
    pub actual_count: i64,
    pub diff: i64,
}

#[derive(Serialize)]
pub struct InventoryDrift {
    pub item_id: i64,
    pub stored_quantity: i32,
    pub txn_sum: i64,
    pub diff: i64,
}

pub async fn reconcile(pool: &PgPool) -> Result<ReconciliationResult, AppError> {
    // 1. Payment-to-order: find orders where pay_status doesn't match payment_record count
    let payment_rows = sqlx::query(
        r#"SELECT ro.id AS order_id, ro.pay_status,
           COUNT(pr.id) AS paid_records,
           SUM(pr.amount) AS total_paid_amount
           FROM reward_order ro
           LEFT JOIN payment_record pr ON pr.order_id = ro.id AND pr.status = 'success'
           GROUP BY ro.id, ro.pay_status
           HAVING (ro.pay_status = 'paid' AND COUNT(pr.id) = 0)
               OR (ro.pay_status != 'paid' AND COUNT(pr.id) > 0)"#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let payment_mismatches: Vec<PaymentMismatch> = payment_rows.iter().map(|r| PaymentMismatch {
        order_id: r.get::<i64, _>("order_id"),
        order_pay_status: r.get::<String, _>("pay_status"),
        paid_records: r.get::<i64, _>("paid_records"),
        total_paid_amount: r.try_get::<f64, _>("total_paid_amount").ok(),
    }).collect();

    // 2. Vote-to-entry: find entries where valid_vote_count != actual valid vote count
    let vote_rows = sqlx::query(
        r#"SELECT e.id AS entry_id, e.valid_vote_count,
           COUNT(v.id) AS actual_count
           FROM contest_entry e
           LEFT JOIN vote_record v ON v.entry_id = e.id AND v.vote_status = 'valid'
           GROUP BY e.id, e.valid_vote_count
           HAVING e.valid_vote_count != COUNT(v.id)
              AND ABS(e.valid_vote_count - COUNT(v.id)::int) > 0"#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let vote_count_drifts: Vec<VoteCountDrift> = vote_rows.iter().map(|r| {
        let stored: i32 = r.get::<i32, _>("valid_vote_count");
        let actual: i64 = r.get::<i64, _>("actual_count");
        VoteCountDrift {
            entry_id: r.get::<i64, _>("entry_id"),
            stored_count: stored,
            actual_count: actual,
            diff: actual - stored as i64,
        }
    }).collect();

    // 3. Inventory drift: find items where stored quantity != sum of transactions
    let inv_rows = sqlx::query(
        r#"SELECT i.id AS item_id, i.quantity AS stored_quantity,
           COALESCE(SUM(CASE WHEN t.txn_type = 'in' THEN t.quantity ELSE -t.quantity END), 0) AS txn_sum
           FROM inventory_item i
           LEFT JOIN inventory_txn t ON t.item_id = i.id
           GROUP BY i.id, i.quantity
           HAVING i.quantity != COALESCE(SUM(CASE WHEN t.txn_type = 'in' THEN t.quantity ELSE -t.quantity END), 0)::int"#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let inventory_drifts: Vec<InventoryDrift> = inv_rows.iter().map(|r| {
        let stored: i32 = r.get::<i32, _>("stored_quantity");
        let txn_sum: i64 = r.get::<i64, _>("txn_sum");
        InventoryDrift {
            item_id: r.get::<i64, _>("item_id"),
            stored_quantity: stored,
            txn_sum,
            diff: txn_sum - stored as i64,
        }
    }).collect();

    Ok(ReconciliationResult {
        payment_mismatches,
        vote_count_drifts,
        inventory_drifts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vote_drift_positive_diff() {
        let stored: i32 = 10;
        let actual: i64 = 13;
        assert_eq!(actual - stored as i64, 3);
    }

    #[test]
    fn vote_drift_negative_diff() {
        let stored: i32 = 15;
        let actual: i64 = 10;
        assert_eq!(actual - stored as i64, -5);
    }

    #[test]
    fn inventory_drift_calculation() {
        let stored: i32 = 100;
        let txn_sum: i64 = 95;
        assert_eq!(txn_sum - stored as i64, -5);
    }

    #[test]
    fn empty_result_is_consistent() {
        let result = ReconciliationResult {
            payment_mismatches: vec![],
            vote_count_drifts: vec![],
            inventory_drifts: vec![],
        };
        assert!(result.payment_mismatches.is_empty());
        assert!(result.vote_count_drifts.is_empty());
        assert!(result.inventory_drifts.is_empty());
    }

    #[test]
    fn payment_mismatch_fields() {
        let m = PaymentMismatch {
            order_id: 42,
            order_pay_status: "pending".to_string(),
            paid_records: 1,
            total_paid_amount: Some(29.9),
        };
        assert_eq!(m.order_id, 42);
        assert_eq!(m.order_pay_status, "pending");
        assert_eq!(m.paid_records, 1);
    }
}
