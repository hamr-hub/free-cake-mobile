use redis::Client;
use crate::errors::AppError;

pub struct RankCacheService;

impl RankCacheService {
    pub async fn get_rank(
        client: &Client,
        activity_id: i64,
    ) -> Result<Vec<(i64, i32)>, AppError> {
        let mut conn = client.get_multiplexed_async_connection().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let key = format!("rank:{}", activity_id);
        let result: Vec<(i64, i32)> = redis::cmd("ZRREVRANGE")
            .arg(&key)
            .arg(0)
            .arg(99)
            .arg("WITHSCORES")
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(result)
    }

    pub async fn update_rank_with_redis(
        client: &Client,
        activity_id: i64,
        entry_id: i64,
        valid_vote_count: i32,
    ) -> Result<(), AppError> {
        let mut conn = client.get_multiplexed_async_connection().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let key = format!("rank:{}", activity_id);
        redis::cmd("ZADD")
            .arg(&key)
            .arg(valid_vote_count)
            .arg(entry_id)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        redis::cmd("EXPIRE")
            .arg(&key)
            .arg(300)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }
}
