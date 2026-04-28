use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::AuditLog;

#[derive(Deserialize)]
pub struct AuditLogQuery {
    pub target_type: Option<String>,
    pub target_id: Option<i64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct AuditLogListResponse {
    pub list: Vec<AuditLog>,
    pub total: i64,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<AuditLogQuery>,
) -> Result<Json<AuditLogListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM audit_log WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM audit_log WHERE 1=1");

    if let Some(ref tt) = params.target_type {
        query_builder.push(" AND target_type = ");
        query_builder.push_bind(tt);
        count_builder.push(" AND target_type = ");
        count_builder.push_bind(tt);
    }
    if let Some(tid) = params.target_id {
        query_builder.push(" AND target_id = ");
        query_builder.push_bind(tid);
        count_builder.push(" AND target_id = ");
        count_builder.push_bind(tid);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total: i64 = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<AuditLog>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(AuditLogListResponse { list, total }))
}

#[derive(Deserialize)]
pub struct SendVerifyCodeRequest {
    pub phone: String,
}

#[derive(Serialize)]
pub struct SendVerifyCodeResponse {
    pub success: bool,
    pub expires_in: i64,
}

pub async fn send_verify_code(
    State(state): State<AppState>,
    Json(req): Json<SendVerifyCodeRequest>,
) -> Result<Json<SendVerifyCodeResponse>, AppError> {
    if req.phone.is_empty() {
        return Err(AppError::BadRequest("Phone is required".into()));
    }

    let rate_key = format!("sms_rate:{}", req.phone);
    let mut conn = state.redis_client.get_multiplexed_async_connection().await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let count: i64 = redis::cmd("INCR")
        .arg(&rate_key)
        .query_async::<i64>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if count == 1 {
        redis::cmd("EXPIRE")
            .arg(&rate_key)
            .arg(3600)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    }
    if count > 5 {
        return Err(AppError::RateLimited("Too many verify code requests".into()));
    }

    let code = format!("{:06}", rand::random::<u32>() % 1000000);
    let verify_key = format!("verify_code:{}:{}", req.phone, code);
    let expires_in = 300;
    redis::cmd("SET")
        .arg(&verify_key)
        .arg(&code)
        .arg("EX")
        .arg(expires_in)
        .query_async::<()>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(SendVerifyCodeResponse { success: true, expires_in }))
}

use crate::db::models::VoteRecord;

#[derive(Deserialize)]
pub struct VoteRiskQuery {
    pub activity_id: Option<i64>,
    pub vote_status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct VoteRiskListResponse {
    pub list: Vec<VoteRecord>,
    pub total: i64,
}

pub async fn vote_risk_list(
    State(state): State<AppState>,
    Query(params): Query<VoteRiskQuery>,
) -> Result<Json<VoteRiskListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT * FROM vote_record WHERE vote_status IN ('frozen', 'invalid')"
    );
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT COUNT(*) FROM vote_record WHERE vote_status IN ('frozen', 'invalid')"
    );

    if let Some(aid) = params.activity_id {
        query_builder.push(" AND activity_id = ");
        query_builder.push_bind(aid);
        count_builder.push(" AND activity_id = ");
        count_builder.push_bind(aid);
    }
    if let Some(ref vs) = params.vote_status {
        query_builder.push(" AND vote_status = ");
        query_builder.push_bind(vs);
        count_builder.push(" AND vote_status = ");
        count_builder.push_bind(vs);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total: i64 = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<VoteRecord>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(VoteRiskListResponse { list, total }))
}

use sqlx::Row;

#[derive(Deserialize)]
pub struct WinnerListQuery {
    pub activity_id: Option<i64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct WinnerWithOrder {
    pub id: i64,
    pub activity_id: i64,
    pub entry_id: i64,
    pub user_id: i64,
    pub rank: i32,
    pub valid_vote_count: i32,
    pub status: String,
    pub created_at: String,
    pub store_id: i64,
    pub production_status: String,
    pub redeem_status: String,
}

#[derive(Serialize)]
pub struct WinnerListResponse {
    pub list: Vec<WinnerWithOrder>,
    pub total: i64,
}

pub async fn winner_list(
    State(state): State<AppState>,
    Query(params): Query<WinnerListQuery>,
) -> Result<Json<WinnerListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT COUNT(*) FROM winner_record w WHERE 1=1"
    );
    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT w.id, w.activity_id, w.entry_id, w.user_id, w.rank, w.valid_vote_count, w.status, w.created_at, r.store_id, r.production_status, r.redeem_status FROM winner_record w LEFT JOIN reward_order r ON r.winner_id = w.id WHERE 1=1"
    );

    if let Some(aid) = params.activity_id {
        count_builder.push(" AND w.activity_id = ");
        count_builder.push_bind(aid);
        query_builder.push(" AND w.activity_id = ");
        query_builder.push_bind(aid);
    }

    query_builder.push(" ORDER BY w.rank ASC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total: i64 = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let rows = query_builder.build()
        .bind(page_size)
        .bind(offset)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = rows.iter().map(|row| WinnerWithOrder {
        id: row.get::<i64, _>("id"),
        activity_id: row.get::<i64, _>("activity_id"),
        entry_id: row.get::<i64, _>("entry_id"),
        user_id: row.get::<i64, _>("user_id"),
        rank: row.get::<i32, _>("rank"),
        valid_vote_count: row.get::<i32, _>("valid_vote_count"),
        status: row.get::<String, _>("status"),
        created_at: row.get::<String, _>("created_at"),
        store_id: row.get::<i64, _>("store_id"),
        production_status: row.get::<String, _>("production_status"),
        redeem_status: row.get::<String, _>("redeem_status"),
    }).collect();

    Ok(Json(WinnerListResponse { list, total }))
}

use crate::db::models::ProductionTask;

#[derive(Deserialize)]
pub struct ProductionListQuery {
    pub store_id: Option<i64>,
    pub task_status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct ProductionListResponse {
    pub list: Vec<ProductionTask>,
    pub total: i64,
}

pub async fn production_list(
    State(state): State<AppState>,
    Query(params): Query<ProductionListQuery>,
) -> Result<Json<ProductionListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM production_task WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM production_task WHERE 1=1");

    if let Some(sid) = params.store_id {
        query_builder.push(" AND store_id = ");
        query_builder.push_bind(sid);
        count_builder.push(" AND store_id = ");
        count_builder.push_bind(sid);
    }
    if let Some(ref ts) = params.task_status {
        query_builder.push(" AND task_status = ");
        query_builder.push_bind(ts);
        count_builder.push(" AND task_status = ");
        count_builder.push_bind(ts);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total: i64 = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<ProductionTask>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(ProductionListResponse { list, total }))
}

use crate::db::models::RedeemCode;

#[derive(Deserialize)]
pub struct RedeemListQuery {
    pub order_id: Option<i64>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct RedeemListResponse {
    pub list: Vec<RedeemCode>,
    pub total: i64,
}

pub async fn redeem_list(
    State(state): State<AppState>,
    Query(params): Query<RedeemListQuery>,
) -> Result<Json<RedeemListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM redeem_code WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM redeem_code WHERE 1=1");

    if let Some(oid) = params.order_id {
        query_builder.push(" AND order_id = ");
        query_builder.push_bind(oid);
        count_builder.push(" AND order_id = ");
        count_builder.push_bind(oid);
    }
    if let Some(ref s) = params.status {
        query_builder.push(" AND status = ");
        query_builder.push_bind(s);
        count_builder.push(" AND status = ");
        count_builder.push_bind(s);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total: i64 = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<RedeemCode>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(RedeemListResponse { list, total }))
}

#[derive(Deserialize)]
pub struct EntryListQuery {
    pub activity_id: Option<i64>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct EntryWithUserInfo {
    pub id: i64,
    pub activity_id: i64,
    pub user_id: i64,
    pub selected_generation_id: i64,
    pub selected_template_id: i64,
    pub title: String,
    pub share_code: String,
    pub image_url: String,
    pub raw_vote_count: i32,
    pub valid_vote_count: i32,
    pub risk_score: f64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_name: String,
    pub region_name: String,
    pub ai_generated: bool,
    pub vote_count: i32,
    pub risk_tags: Option<String>,
}

#[derive(Serialize)]
pub struct EntryListResponse {
    pub list: Vec<EntryWithUserInfo>,
    pub total: i64,
}

pub async fn entry_list(
    State(state): State<AppState>,
    Query(params): Query<EntryListQuery>,
) -> Result<Json<EntryListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT COUNT(*) FROM contest_entry e WHERE 1=1"
    );
    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT e.*, u.nickname AS user_name, r.name AS region_name FROM contest_entry e LEFT JOIN app_user u ON e.user_id = u.id LEFT JOIN activity a ON e.activity_id = a.id LEFT JOIN region r ON a.region_id = r.id WHERE 1=1"
    );

    if let Some(aid) = params.activity_id {
        count_builder.push(" AND e.activity_id = ");
        count_builder.push_bind(aid);
        query_builder.push(" AND e.activity_id = ");
        query_builder.push_bind(aid);
    }
    if let Some(ref s) = params.status {
        count_builder.push(" AND e.status = ");
        count_builder.push_bind(s);
        query_builder.push(" AND e.status = ");
        query_builder.push_bind(s);
    }

    query_builder.push(" ORDER BY e.created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total: i64 = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let rows = query_builder.build()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = rows.iter().map(|row| EntryWithUserInfo {
        id: row.get::<i64, _>("id"),
        activity_id: row.get::<i64, _>("activity_id"),
        user_id: row.get::<i64, _>("user_id"),
        selected_generation_id: row.get::<i64, _>("selected_generation_id"),
        selected_template_id: row.get::<i64, _>("selected_template_id"),
        title: row.get::<String, _>("title"),
        share_code: row.get::<String, _>("share_code"),
        image_url: row.get::<String, _>("image_url"),
        raw_vote_count: row.get::<i32, _>("raw_vote_count"),
        valid_vote_count: row.get::<i32, _>("valid_vote_count"),
        risk_score: row.get::<f64, _>("risk_score"),
        status: row.get::<String, _>("status"),
        created_at: row.get::<String, _>("created_at"),
        updated_at: row.get::<String, _>("updated_at"),
        user_name: row.get::<String, _>("user_name"),
        region_name: row.get::<String, _>("region_name"),
        ai_generated: row.get::<i64, _>("selected_generation_id") > 0,
        vote_count: row.get::<i32, _>("valid_vote_count"),
        risk_tags: None,
    }).collect();

    Ok(Json(EntryListResponse { list, total }))
}

use crate::db::models::RiskEvent;

#[derive(Deserialize)]
pub struct RiskEventListQuery {
    pub activity_id: Option<i64>,
    pub risk_type: Option<String>,
    pub risk_level: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct RiskEventListResponse {
    pub list: Vec<RiskEvent>,
    pub total: i64,
}

pub async fn risk_event_list(
    State(state): State<AppState>,
    Query(params): Query<RiskEventListQuery>,
) -> Result<Json<RiskEventListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM risk_event WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM risk_event WHERE 1=1");

    if let Some(aid) = params.activity_id {
        query_builder.push(" AND activity_id = ");
        query_builder.push_bind(aid);
        count_builder.push(" AND activity_id = ");
        count_builder.push_bind(aid);
    }
    if let Some(ref rt) = params.risk_type {
        query_builder.push(" AND risk_type = ");
        query_builder.push_bind(rt);
        count_builder.push(" AND risk_type = ");
        count_builder.push_bind(rt);
    }
    if let Some(ref rl) = params.risk_level {
        query_builder.push(" AND risk_level = ");
        query_builder.push_bind(rl);
        count_builder.push(" AND risk_level = ");
        query_builder.push_bind(rl);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total: i64 = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<RiskEvent>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(RiskEventListResponse { list, total }))
}
