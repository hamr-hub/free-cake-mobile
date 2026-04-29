use axum::{extract::{State, Path, Extension, Query}, Json};
use serde::{Serialize, Deserialize};
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub vote_status: String,
}

#[derive(Serialize)]
pub struct UserRedeemCode {
    pub code: String,
    pub status: String,
    pub order_id: i64,
    pub store_address: String,
    pub cake_size: String,
    pub cream_type: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
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
        created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at"),
        vote_status: r.get::<String, _>("vote_status"),
    }).collect();

    // User's redeem codes
    let code_rows = sqlx::query(
        r#"SELECT rc.code, CASE
           WHEN rc.status = 'used' THEN 'used'
           WHEN rc.expires_at < CURRENT_TIMESTAMP THEN 'expired'
           ELSE 'unused'
           END AS status,
           rc.order_id, rc.expires_at,
           s.address AS store_address,
           COALESCE(ar.cake_size, '6inch') AS cake_size,
           COALESCE(ar.cream_type, 'animal') AS cream_type
           FROM redeem_code rc
           JOIN reward_order ro ON rc.order_id = ro.id
           JOIN winner_record wr ON ro.winner_id = wr.id
           JOIN store s ON ro.store_id = s.id
           LEFT JOIN activity a ON wr.activity_id = a.id
           LEFT JOIN activity_rule ar ON ar.activity_id = a.id
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
        order_id: r.get::<i64, _>("order_id"),
        store_address: r.get::<String, _>("store_address"),
        cake_size: r.get::<String, _>("cake_size"),
        cream_type: r.get::<String, _>("cream_type"),
        expires_at: r.get::<chrono::DateTime<chrono::Utc>, _>("expires_at"),
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
    pub expires_at: chrono::DateTime<chrono::Utc>,
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
    let expires_at: chrono::DateTime<chrono::Utc> = row.get("expires_at");
    let display_status = if code_status == "used" {
        "used".to_string()
    } else if expires_at < chrono::Utc::now() {
        "expired".to_string()
    } else {
        "unused".to_string()
    };

    Ok(Json(RedeemDetailResponse {
        code: row.get("code"),
        status: display_status,
        order_id: row.get("order_id"),
        store_address: row.get("store_address"),
        cake_size: row.get("cake_size"),
        cream_type: row.get("cream_type"),
        expires_at,
    }))
}

#[derive(Deserialize)]
pub struct ResolveRegionQuery {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize)]
pub struct ResolveRegionResponse {
    pub region_id: Option<i64>,
    pub region_name: Option<String>,
    pub distance_km: Option<f64>,
    pub is_in_range: bool,
}

pub async fn resolve_region(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<ResolveRegionQuery>,
) -> Result<Json<ResolveRegionResponse>, AppError> {
    let row = sqlx::query(
        r#"SELECT id, name,
           ACOS(COS(RADIANS($1)) * COS(RADIANS(center_lat)) * COS(RADIANS(center_lng) - RADIANS($2)) + SIN(RADIANS($1)) * SIN(RADIANS(center_lat))) * 6371.0 AS distance_km
           FROM region
           WHERE status = 'active'
           AND ACOS(COS(RADIANS($1)) * COS(RADIANS(center_lat)) * COS(RADIANS(center_lng) - RADIANS($2)) + SIN(RADIANS($1)) * SIN(RADIANS(center_lat))) * 6371.0 <= coverage_radius_km
           ORDER BY distance_km
           LIMIT 1"#
    )
    .bind(query.lat)
    .bind(query.lng)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    if let Some(row) = row {
        let region_id: i64 = row.get("id");
        let region_name: String = row.get("name");
        let distance_km: f64 = row.get("distance_km");

        let _ = sqlx::query("UPDATE app_user SET region_id = $1 WHERE id = $2")
            .bind(region_id)
            .bind(claims.user_id)
            .execute(&state.db_pool)
            .await;

        Ok(Json(ResolveRegionResponse {
            region_id: Some(region_id),
            region_name: Some(region_name),
            distance_km: Some(distance_km),
            is_in_range: true,
        }))
    } else {
        Ok(Json(ResolveRegionResponse {
            region_id: None,
            region_name: None,
            distance_km: None,
            is_in_range: false,
        }))
    }
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateProfileResponse {
    pub id: i64,
    pub nickname: String,
}

pub async fn update_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UpdateProfileResponse>, AppError> {
    if let Some(ref nickname) = req.nickname {
        if nickname.trim().is_empty() || nickname.len() > 32 {
            return Err(AppError::BadRequest("Nickname must be 1-32 characters".into()));
        }
    }

    let row = sqlx::query(
        "UPDATE app_user SET nickname = COALESCE($1, nickname) WHERE id = $2 RETURNING id, nickname"
    )
    .bind(req.nickname.as_deref())
    .bind(claims.user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(Json(UpdateProfileResponse {
        id: row.get("id"),
        nickname: row.get("nickname"),
    }))
}

#[derive(Serialize)]
pub struct MyEntriesResponse {
    pub list: Vec<UserEntry>,
    pub total: i64,
}

pub async fn my_entries(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<crate::handlers::query_handlers::EntryListQuery>,
) -> Result<Json<MyEntriesResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM contest_entry WHERE user_id = $1")
        .bind(claims.user_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let rows = sqlx::query(
        r#"SELECT e.id, e.title, e.valid_vote_count,
           RANK() OVER (ORDER BY e.valid_vote_count DESC) AS rank,
           EXISTS(SELECT 1 FROM winner_record w WHERE w.entry_id = e.id) AS is_winner
           FROM contest_entry e WHERE e.user_id = $1 AND e.status = 'active'
           ORDER BY e.created_at DESC LIMIT $2 OFFSET $3"#
    )
    .bind(claims.user_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = rows.iter().map(|r| UserEntry {
        id: r.get::<i64, _>("id"),
        title: r.get::<String, _>("title"),
        valid_vote_count: r.get::<i32, _>("valid_vote_count"),
        rank: r.get::<i32, _>("rank"),
        is_winner: r.get::<bool, _>("is_winner"),
    }).collect();

    Ok(Json(MyEntriesResponse { list, total: count }))
}

#[derive(Serialize)]
pub struct MyVotesResponse {
    pub list: Vec<UserVote>,
    pub total: i64,
}

pub async fn my_votes(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<crate::handlers::query_handlers::EntryListQuery>,
) -> Result<Json<MyVotesResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM vote_record WHERE voter_user_id = $1")
        .bind(claims.user_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let rows = sqlx::query(
        "SELECT v.id, v.entry_id, v.created_at, v.vote_status FROM vote_record v WHERE v.voter_user_id = $1 ORDER BY v.created_at DESC LIMIT $2 OFFSET $3"
    )
    .bind(claims.user_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = rows.iter().map(|r| UserVote {
        id: r.get::<i64, _>("id"),
        entry_id: r.get::<i64, _>("entry_id"),
        created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at"),
        vote_status: r.get::<String, _>("vote_status"),
    }).collect();

    Ok(Json(MyVotesResponse { list, total: count }))
}

#[derive(Serialize)]
pub struct MyOrder {
    pub id: i64,
    pub order_type: Option<String>,
    pub amount: Option<f64>,
    pub pay_status: Option<String>,
    pub refund_status: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct MyOrdersResponse {
    pub list: Vec<MyOrder>,
    pub total: i64,
}

pub async fn my_orders(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<crate::handlers::query_handlers::EntryListQuery>,
) -> Result<Json<MyOrdersResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reward_order WHERE user_id = $1")
        .bind(claims.user_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let rows = sqlx::query(
        "SELECT id, order_type, amount, pay_status, refund_status, created_at FROM reward_order WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
    )
    .bind(claims.user_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = rows.iter().map(|r| MyOrder {
        id: r.get::<i64, _>("id"),
        order_type: r.try_get::<String, _>("order_type").ok(),
        amount: r.try_get::<f64, _>("amount").ok(),
        pay_status: r.try_get::<String, _>("pay_status").ok(),
        refund_status: r.try_get::<String, _>("refund_status").ok(),
        created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at"),
    }).collect();

    Ok(Json(MyOrdersResponse { list, total: count }))
}
