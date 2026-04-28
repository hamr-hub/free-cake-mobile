use axum::{extract::{State, Path, Query, Extension}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::ContestEntry;
use crate::services::risk_control::RiskControlService;
use crate::services::rank_cache::RankCacheService;
use crate::app_middleware::auth::Claims;

#[derive(Deserialize)]
pub struct CastVoteRequest {
    pub activity_id: i64,
    pub voter_phone_hash: Option<String>,
    pub voter_device_id: Option<String>,
    pub voter_ip: Option<String>,
    pub geohash: Option<String>,
}

#[derive(Serialize)]
pub struct CastVoteResponse {
    pub vote_id: i64,
    pub vote_status: String,
    pub remaining_votes: i32,
}

#[derive(Deserialize)]
pub struct RankQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct RankResponse {
    pub entries: Vec<ContestEntry>,
    pub total: i64,
}

pub async fn cast(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(entry_id): Path<i64>,
    Json(req): Json<CastVoteRequest>,
) -> Result<Json<CastVoteResponse>, AppError> {
    let activity = sqlx::query("SELECT status FROM activity WHERE id = ?")
        .bind(req.activity_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = activity.ok_or_else(|| AppError::NotFound("Activity not found".into()))?;
    let status: String = row.get("status");
    if status != "voting_open" {
        return Err(AppError::BadRequest("Activity is not in voting phase".into()));
    }

    let entry = sqlx::query("SELECT id, activity_id FROM contest_entry WHERE id = ? AND status = 'active'")
        .bind(entry_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if entry.is_none() {
        return Err(AppError::NotFound("Entry not found or inactive".into()));
    }

    let daily_key = format!("daily_votes:{}:{}", req.activity_id, claims.user_id);
    let mut conn = state.redis_client.get_multiplexed_async_connection().await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let count: i64 = redis::cmd("INCR")
        .arg(&daily_key)
        .query_async::<i64>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if count == 1 {
        redis::cmd("EXPIRE")
            .arg(&daily_key)
            .arg(86400)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    }
    let max_votes = state.config.max_votes_per_day as i64;
    if count > max_votes {
        return Err(AppError::RateLimited("Daily vote limit exceeded".into()));
    }

    let phone_hash = req.voter_phone_hash.as_deref().unwrap_or("");
    let device_id = req.voter_device_id.as_deref().unwrap_or("");
    let ip = req.voter_ip.as_deref().unwrap_or("");
    let geohash = req.geohash.as_deref().unwrap_or("");

    let (is_risky, risk_tags) = RiskControlService::check_vote_risk(
        &state.db_pool,
        phone_hash, device_id, ip, geohash, req.activity_id,
    ).await?;

    let vote_status = if is_risky { "frozen" } else { "valid" };
    let risk_tags_str = serde_json::to_string(&risk_tags).unwrap_or_default();

    let result = sqlx::query(
        "INSERT INTO vote_record (activity_id, entry_id, voter_user_id, voter_phone_hash, voter_device_id, ip, geohash, vote_status, risk_tags) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(req.activity_id)
    .bind(entry_id)
    .bind(claims.user_id)
    .bind(phone_hash)
    .bind(device_id)
    .bind(ip)
    .bind(geohash)
    .bind(vote_status)
    .bind(&risk_tags_str)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    if vote_status == "valid" {
        sqlx::query("UPDATE contest_entry SET raw_vote_count = raw_vote_count + 1, valid_vote_count = valid_vote_count + 1 WHERE id = ?")
            .bind(entry_id)
            .execute(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let entry_row = sqlx::query("SELECT valid_vote_count FROM contest_entry WHERE id = ?")
            .bind(entry_id)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let valid_count: i32 = entry_row.get("valid_vote_count");

        RankCacheService::update_rank_with_redis(&state.redis_client, req.activity_id, entry_id, valid_count).await?;
    } else {
        sqlx::query("UPDATE contest_entry SET raw_vote_count = raw_vote_count + 1 WHERE id = ?")
            .bind(entry_id)
            .execute(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let remaining = max_votes - count;

    Ok(Json(CastVoteResponse {
        vote_id: result.last_insert_id() as i64,
        vote_status: vote_status.to_string(),
        remaining_votes: remaining as i32,
    }))
}

pub async fn rank(
    State(state): State<AppState>,
    Path(activity_id): Path<i64>,
    Query(params): Query<RankQuery>,
) -> Result<Json<RankResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM contest_entry WHERE activity_id = ? AND status = 'active'"
    )
    .bind(activity_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let entries = sqlx::query_as::<_, ContestEntry>(
        "SELECT * FROM contest_entry WHERE activity_id = ? AND status = 'active' ORDER BY valid_vote_count DESC LIMIT ? OFFSET ?"
    )
    .bind(activity_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(RankResponse { entries, total }))
}
