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

    let mut where_clauses = vec!["1=1".to_string()];
    if let Some(ref tt) = params.target_type {
        where_clauses.push(format!("target_type = '{}'", tt));
    }
    if let Some(tid) = params.target_id {
        where_clauses.push(format!("target_id = {}", tid));
    }
    let where_str = where_clauses.join(" AND ");

    let total = sqlx::query_scalar::<_, i64>(
        &format!("SELECT COUNT(*) FROM audit_log WHERE {}", where_str)
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = sqlx::query_as::<_, AuditLog>(
        &format!("SELECT * FROM audit_log WHERE {} ORDER BY created_at DESC LIMIT ? OFFSET ?", where_str)
    )
    .bind(page_size)
    .bind(offset)
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

    let mut where_clauses = vec!["vote_status IN ('frozen', 'invalid')".to_string()];
    if let Some(ref aid) = params.activity_id {
        where_clauses.push(format!("activity_id = {}", aid));
    }
    if let Some(ref vs) = params.vote_status {
        where_clauses.push(format!("vote_status = '{}'", vs));
    }
    let where_str = where_clauses.join(" AND ");

    let total = sqlx::query_scalar::<_, i64>(
        &format!("SELECT COUNT(*) FROM vote_record WHERE {}", where_str)
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = sqlx::query_as::<_, VoteRecord>(
        &format!("SELECT * FROM vote_record WHERE {} ORDER BY created_at DESC LIMIT ? OFFSET ?", where_str)
    )
    .bind(page_size)
    .bind(offset)
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

    let mut where_clauses = vec!["1=1".to_string()];
    if let Some(ref aid) = params.activity_id {
        where_clauses.push(format!("w.activity_id = {}", aid));
    }
    let where_str = where_clauses.join(" AND ");

    let total = sqlx::query_scalar::<_, i64>(
        &format!("SELECT COUNT(*) FROM winner_record w WHERE {}", where_str)
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let rows = sqlx::query(
        &format!(
            "SELECT w.id, w.activity_id, w.entry_id, w.user_id, w.rank, w.valid_vote_count, w.status, w.created_at, r.store_id, r.production_status, r.redeem_status FROM winner_record w LEFT JOIN reward_order r ON r.winner_id = w.id WHERE {} ORDER BY w.rank ASC LIMIT ? OFFSET ?",
            where_str
        )
    )
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

    let mut where_clauses = vec!["1=1".to_string()];
    if let Some(ref sid) = params.store_id {
        where_clauses.push(format!("store_id = {}", sid));
    }
    if let Some(ref ts) = params.task_status {
        where_clauses.push(format!("task_status = '{}'", ts));
    }
    let where_str = where_clauses.join(" AND ");

    let total = sqlx::query_scalar::<_, i64>(
        &format!("SELECT COUNT(*) FROM production_task WHERE {}", where_str)
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = sqlx::query_as::<_, ProductionTask>(
        &format!("SELECT * FROM production_task WHERE {} ORDER BY created_at DESC LIMIT ? OFFSET ?", where_str)
    )
    .bind(page_size)
    .bind(offset)
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

    let mut where_clauses = vec!["1=1".to_string()];
    if let Some(ref oid) = params.order_id {
        where_clauses.push(format!("order_id = {}", oid));
    }
    if let Some(ref s) = params.status {
        where_clauses.push(format!("status = '{}'", s));
    }
    let where_str = where_clauses.join(" AND ");

    let total = sqlx::query_scalar::<_, i64>(
        &format!("SELECT COUNT(*) FROM redeem_code WHERE {}", where_str)
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = sqlx::query_as::<_, RedeemCode>(
        &format!("SELECT * FROM redeem_code WHERE {} ORDER BY created_at DESC LIMIT ? OFFSET ?", where_str)
    )
    .bind(page_size)
    .bind(offset)
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

    let mut where_clauses = vec!["1=1".to_string()];
    if let Some(ref aid) = params.activity_id {
        where_clauses.push(format!("e.activity_id = {}", aid));
    }
    if let Some(ref s) = params.status {
        where_clauses.push(format!("e.status = '{}'", s));
    }
    let where_str = where_clauses.join(" AND ");

    let total = sqlx::query_scalar::<_, i64>(
        &format!("SELECT COUNT(*) FROM contest_entry e WHERE {}", where_str)
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let rows = sqlx::query(
        &format!(
            "SELECT e.*, u.nickname AS user_name, r.name AS region_name FROM contest_entry e LEFT JOIN user u ON e.user_id = u.id LEFT JOIN activity a ON e.activity_id = a.id LEFT JOIN region r ON a.region_id = r.id WHERE {} ORDER BY e.created_at DESC LIMIT ? OFFSET ?",
            where_str
        )
    )
    .bind(page_size)
    .bind(offset)
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
