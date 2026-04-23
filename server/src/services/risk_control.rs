use crate::errors::AppError;

pub struct RiskControlService;

impl RiskControlService {
    pub async fn check_vote_risk(
        pool: &sqlx::MySqlPool,
        voter_phone_hash: &str,
        voter_device_id: &str,
        voter_ip: &str,
        _geohash: &str,
        activity_id: i64,
    ) -> Result<(bool, Vec<String>), AppError> {
        let mut risk_tags = Vec::new();

        let phone_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM vote_record WHERE voter_phone_hash = ? AND activity_id = ? AND vote_status = 'valid'"
        )
        .bind(voter_phone_hash)
        .bind(activity_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        if phone_count > 10 {
            risk_tags.push("same_phone_cluster".to_string());
        }

        if !voter_device_id.is_empty() {
            let device_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM vote_record WHERE voter_device_id = ? AND activity_id = ?"
            )
            .bind(voter_device_id)
            .bind(activity_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            if device_count > 3 {
                risk_tags.push("same_device_cluster".to_string());
            }
        }

        if !voter_ip.is_empty() {
            let ip_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM vote_record WHERE ip = ? AND activity_id = ?"
            )
            .bind(voter_ip)
            .bind(activity_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            if ip_count > 5 {
                risk_tags.push("ip_cluster".to_string());
            }
        }

        let is_risky = !risk_tags.is_empty();
        Ok((is_risky, risk_tags))
    }
}
