use crate::errors::AppError;

pub struct RiskControlService;

impl RiskControlService {
    #[allow(clippy::too_many_arguments)]
    pub async fn check_vote_risk(
        pool: &sqlx::PgPool,
        risk_enabled: bool,
        voter_phone_hash: &str,
        voter_device_id: &str,
        voter_ip: &str,
        geohash: &str,
        voter_openid: &str,
        activity_id: i64,
    ) -> Result<(bool, Vec<String>), AppError> {
        if !risk_enabled {
            return Ok((false, vec![]));
        }

        let mut risk_tags = Vec::new();
        let mut risk_level = "low".to_string();

        let phone_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM vote_record WHERE voter_phone_hash = $1 AND activity_id = $2 AND vote_status = 'valid'"
        )
        .bind(voter_phone_hash)
        .bind(activity_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
        if phone_count > 10 {
            risk_tags.push("same_phone_cluster".to_string());
            risk_level = "high".to_string();
        }

        if !voter_device_id.is_empty() {
            let device_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM vote_record WHERE voter_device_id = $1 AND activity_id = $2"
            )
            .bind(voter_device_id)
            .bind(activity_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            if device_count > 3 {
                risk_tags.push("same_device_cluster".to_string());
                if risk_level != "high" { risk_level = "medium".to_string(); }
            }
        }

        if !voter_ip.is_empty() {
            let ip_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM vote_record WHERE ip = $1 AND activity_id = $2"
            )
            .bind(voter_ip)
            .bind(activity_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            if ip_count > 5 {
                risk_tags.push("ip_cluster".to_string());
                if risk_level != "high" { risk_level = "medium".to_string(); }
            }
        }

        if !geohash.is_empty() && geohash.len() >= 5 {
            let prefix = &geohash[..5];
            let geo_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM vote_record WHERE geohash LIKE $1 AND activity_id = $2"
            )
            .bind(format!("{}%", prefix))
            .bind(activity_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            if geo_count > 8 {
                risk_tags.push("geo_cluster".to_string());
                if risk_level != "high" { risk_level = "medium".to_string(); }
            }
        }

        if !voter_openid.is_empty() {
            let openid_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM vote_record WHERE voter_phone_hash = $1 AND activity_id = $2 AND voter_device_id = $3 AND ip = $4"
            )
            .bind(voter_phone_hash)
            .bind(activity_id)
            .bind(voter_device_id)
            .bind(voter_ip)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            if openid_count > 1 {
                risk_tags.push("openid_duplicate".to_string());
            }
        }

        let is_risky = !risk_tags.is_empty();

        if is_risky {
            let risk_tags_str = serde_json::to_string(&risk_tags).unwrap_or_default();
            let _ = sqlx::query(
                "INSERT INTO risk_event (activity_id, entry_id, risk_type, risk_level, description, status) VALUES ($1, 0, 'vote_risk', $2, $3, 'open')"
            )
            .bind(activity_id)
            .bind(&risk_level)
            .bind(&risk_tags_str)
            .execute(pool)
            .await;
        }

        Ok((is_risky, risk_tags))
    }
}
