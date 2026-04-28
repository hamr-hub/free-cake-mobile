use axum::{extract::{State, Path, Extension}, Json};
use serde::Serialize;
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::app_middleware::auth::Claims;

#[derive(Serialize)]
pub struct UserProfileResponse {
    pub user: UserBasic,
    pub entries: Vec<UserEntry>,
    pub votes: Vec<UserVote>,
    pub redeem_codes: Vec<UserRedeemCode>,
}

#[derive(Serialize)]
pub struct UserBasic {
    pub id: i64,
    pub nickname: String,
    pub phone: String,
    pub region_name: String,
}

#[derive(Serialize)]
pub struct UserEntry {
    pub id: i64,
    pub title: String,
    pub valid_vote_count: i32,
    pub rank: i32,
    pub is_winner: bool,
}

#[derive(Serialize)]
pub struct UserVote {
    pub id: i64,
    pub entry_id: i64,
    pub created_at: String,
    pub vote_status: String,
}

#[derive(Serialize)]
pub struct UserRedeemCode {
    pub code: String,
    pub status: String,
}

pub async fn me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserProfileResponse>, AppError> {
    let user_row = sqlx::query(
        "SELECT u.id, u.nickname, u.phone, COALESCE(r.name, '未知赛区') AS region_name FROM app_user u LEFT JOIN region r ON u.region_id = r.id WHERE u.id = $1"
    )
    .bind(claims.user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    let user = UserBasic {
        id: user_row.get::<i64, _>("id"),
        nickname: user_row.get::<String, _>("nickname"),
        phone: user_row.get::<String, _>("phone"),
        region_name: user_row.get::<String, _>("region_name"),
    };

    // User's entries with rank
    let entry_rows = sqlx::query(
        r#"SELECT e.id, e.title, e.valid_vote_count,
           RANK() OVER (ORDER BY e.valid_vote_count DESC) AS rank,
           EXISTS(SELECT 1 FROM winner_record w WHERE w.entry_id = e.id) AS is_winner
           FROM contest_entry e WHERE e.user_id = $1 AND e.status = 'active'
           ORDER BY e.created_at DESC"#
    )
    .bind(claims.user_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let entries = entry_rows.iter().map(|r| UserEntry {
        id: r.get::<i64, _>("id"),
        title: r.get::<String, _>("title"),
        valid_vote_count: r.get::<i32, _>("valid_vote_count"),
        rank: r.get::<i32, _>("rank"),
        is_winner: r.get::<bool, _>("is_winner"),
    }).collect();

    // User's recent votes
    let vote_rows = sqlx::query(
        "SELECT v.id, v.entry_id, v.created_at, v.vote_status FROM vote_record v WHERE v.voter_user_id = $1 ORDER BY v.created_at DESC LIMIT 50"
    )
    .bind(claims.user_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let votes = vote_rows.iter().map(|r| UserVote {
        id: r.get::<i64, _>("id"),
        entry_id: r.get::<i64, _>("entry_id"),
        created_at: r.get::<String, _>("created_at"),
        vote_status: r.get::<String, _>("vote_status"),
    }).collect();

    // User's redeem codes
    let code_rows = sqlx::query(
        r#"SELECT rc.code, CASE
           WHEN rc.status = 'used' THEN 'used'
           WHEN rc.expires_at < CURRENT_TIMESTAMP THEN 'expired'
           ELSE 'unused'
           END AS status
           FROM redeem_code rc
           JOIN reward_order ro ON rc.order_id = ro.id
           JOIN winner_record wr ON ro.winner_id = wr.id
           WHERE wr.user_id = $1
           ORDER BY rc.created_at DESC"#
    )
    .bind(claims.user_id)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let redeem_codes = code_rows.iter().map(|r| UserRedeemCode {
        code: r.get::<String, _>("code"),
        status: r.get::<String, _>("status"),
    }).collect();

    Ok(Json(UserProfileResponse { user, entries, votes, redeem_codes }))
}

#[derive(Serialize)]
pub struct RedeemDetailResponse {
    pub code: String,
    pub status: String,
    pub order_id: i64,
    pub store_address: String,
    pub cake_size: String,
    pub cream_type: String,
    pub expires_at: String,
}

pub async fn redeem_detail(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<RedeemDetailResponse>, AppError> {
    let row = sqlx::query(
        r#"SELECT rc.code, rc.status, rc.order_id, rc.expires_at,
           s.address AS store_address,
           COALESCE(ar.cake_size, '6inch') AS cake_size,
           COALESCE(ar.cream_type, 'animal') AS cream_type
           FROM redeem_code rc
           JOIN reward_order ro ON rc.order_id = ro.id
           JOIN store s ON ro.store_id = s.id
           LEFT JOIN winner_record wr ON ro.winner_id = wr.id
           LEFT JOIN activity a ON wr.activity_id = a.id
           LEFT JOIN activity_rule ar ON ar.activity_id = a.id
           WHERE rc.code = $1"#
    )
    .bind(&code)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .ok_or_else(|| AppError::NotFound("Redeem code not found".into()))?;

    let code_status: String = row.get("status");
    let expires_at: chrono::NaiveDateTime = row.get("expires_at");
    let display_status = if code_status == "used" {
        "used".to_string()
    } else if expires_at < chrono::Utc::now().naive_utc() {
        "expired".to_string()
    } else {
        "valid".to_string()
    };

    Ok(Json(RedeemDetailResponse {
        code: row.get("code"),
        status: display_status,
        order_id: row.get("order_id"),
        store_address: row.get("store_address"),
        cake_size: row.get("cake_size"),
        cream_type: row.get("cream_type"),
        expires_at: expires_at.to_string(),
    }))
}
