use axum::{extract::{State, Path, Query, Extension}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::services::risk_control::RiskControlService;
use crate::services::rank_cache::RankCacheService;
use crate::app_middleware::auth::{Claims, ClientIp};

#[derive(Deserialize)]
pub struct CastVoteRequest {
    pub activity_id: i64,
    pub voter_phone_hash: Option<String>,
    pub voter_device_id: Option<String>,
    pub geohash: Option<String>,
    pub voter_openid: Option<String>,
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
pub struct RankedEntry {
    pub id: i64,
    pub title: String,
    pub image_url: Option<String>,
    pub user_name: String,
    pub valid_vote_count: i32,
    pub rank: i32,
    pub is_winner: bool,
}

#[derive(Serialize)]
pub struct RankResponse {
    pub entries: Vec<RankedEntry>,
    pub total: i64,
}

pub async fn cast(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Extension(ClientIp(client_ip)): Extension<ClientIp>,
    Path(entry_id): Path<i64>,
    Json(req): Json<CastVoteRequest>,
) -> Result<Json<CastVoteResponse>, AppError> {
    let activity = sqlx::query("SELECT status FROM activity WHERE id = $1")
        .bind(req.activity_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = activity.ok_or_else(|| AppError::NotFound("Activity not found".into()))?;
    let status: String = row.get("status");
    if status != "voting_open" {
        return Err(AppError::BadRequest("Activity is not in voting phase".into()));
    }

    let entry = sqlx::query("SELECT id, activity_id FROM contest_entry WHERE id = $1 AND status = 'active'")
        .bind(entry_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let entry_row = entry.ok_or_else(|| AppError::NotFound("Entry not found or inactive".into()))?;
    let entry_activity_id: i64 = entry_row.get("activity_id");
    if entry_activity_id != req.activity_id {
        return Err(AppError::BadRequest("Entry does not belong to the specified activity".into()));
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
    let ip = client_ip.as_str();
    let geohash = req.geohash.as_deref().unwrap_or("");
    let openid = req.voter_openid.as_deref().unwrap_or("");

    let (is_risky, risk_tags) = RiskControlService::check_vote_risk(
        &state.db_pool,
        state.config.risk_control_enabled,
        phone_hash, device_id, ip, geohash, openid, req.activity_id,
    ).await?;

    let vote_status = if is_risky { "frozen" } else { "valid" };
    let risk_tags_str = serde_json::to_string(&risk_tags).unwrap_or_default();

    let row = sqlx::query(
        "INSERT INTO vote_record (activity_id, entry_id, voter_user_id, voter_phone_hash, voter_device_id, ip, geohash, vote_status, risk_tags) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id"
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
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    if vote_status == "valid" {
        sqlx::query("UPDATE contest_entry SET raw_vote_count = raw_vote_count + 1, valid_vote_count = valid_vote_count + 1 WHERE id = $1")
            .bind(entry_id)
            .execute(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let entry_row = sqlx::query("SELECT valid_vote_count FROM contest_entry WHERE id = $1")
            .bind(entry_id)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let valid_count: i32 = entry_row.get("valid_vote_count");

        if let Err(e) = RankCacheService::update_rank_with_redis(&state.redis_client, req.activity_id, entry_id, valid_count).await {
            tracing::warn!("Failed to update rank cache for activity {}: {}", req.activity_id, e);
        }
    } else {
        sqlx::query("UPDATE contest_entry SET raw_vote_count = raw_vote_count + 1 WHERE id = $1")
            .bind(entry_id)
            .execute(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let remaining = max_votes - count;

    Ok(Json(CastVoteResponse {
        vote_id: row.get::<i64, _>("id"),
        vote_status: vote_status.to_string(),
        remaining_votes: remaining as i32,
    }))
}

pub async fn rank(
    State(state): State<AppState>,
    Path(activity_id): Path<i64>,
    Query(params): Query<RankQuery>,
) -> Result<Json<RankResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let cache_result = RankCacheService::get_rank(&state.redis_client, activity_id).await;
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM contest_entry WHERE activity_id = $1 AND status = 'active'"
    )
    .bind(activity_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let rank_sql = r#"
        SELECT e.id, e.title, e.image_url, e.valid_vote_count,
               COALESCE(u.nickname, '匿名用户') AS user_name,
               RANK() OVER (ORDER BY e.valid_vote_count DESC) AS rank,
               EXISTS(SELECT 1 FROM winner_record w WHERE w.entry_id = e.id) AS is_winner
        FROM contest_entry e
        LEFT JOIN app_user u ON e.user_id = u.id
        WHERE e.activity_id = $1 AND e.status = 'active'
        ORDER BY e.valid_vote_count DESC
        LIMIT $2 OFFSET $3
    "#;

    if let Ok(cached_ids) = cache_result {
        if !cached_ids.is_empty() && page == 1 && page_size >= cached_ids.len() as i64 {
            let ids: Vec<i64> = cached_ids.iter().map(|(id, _)| *id).collect();
            let rows = sqlx::query(rank_sql)
                .bind(activity_id)
                .bind(page_size)
                .bind(0i64)
                .bind(ids.as_slice())
                .fetch_all(&state.db_pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            let entries = rows.iter().map(|r| RankedEntry {
                id: r.get::<i64, _>("id"),
                title: r.get::<String, _>("title"),
                image_url: r.try_get::<String, _>("image_url").ok(),
                user_name: r.get::<String, _>("user_name"),
                valid_vote_count: r.get::<i32, _>("valid_vote_count"),
                rank: r.get::<i32, _>("rank"),
                is_winner: r.get::<bool, _>("is_winner"),
            }).collect();
            return Ok(Json(RankResponse { entries, total }));
        }
    }

    let rows = sqlx::query(rank_sql)
        .bind(activity_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let entries = rows.iter().map(|r| RankedEntry {
        id: r.get::<i64, _>("id"),
        title: r.get::<String, _>("title"),
        image_url: r.try_get::<String, _>("image_url").ok(),
        user_name: r.get::<String, _>("user_name"),
        valid_vote_count: r.get::<i32, _>("valid_vote_count"),
        rank: r.get::<i32, _>("rank"),
        is_winner: r.get::<bool, _>("is_winner"),
    }).collect();

    Ok(Json(RankResponse { entries, total }))
}
