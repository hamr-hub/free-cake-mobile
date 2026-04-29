use axum::{extract::{State, Path, Extension, Query}, Json};
use axum::http::HeaderMap;
use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::services::{audit_log::AuditLogService, validation, wechat_pay};
use crate::app_middleware::auth::Claims;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ScheduleRequest {
    pub store_id: i64,
    pub scheduled_date: String,
    pub priority: Option<i32>,
}

#[derive(Serialize)]
pub struct ScheduleResponse {
    pub batch_id: i64,
    pub task_ids: Vec<i64>,
}

pub async fn schedule(
    State(state): State<AppState>,
    Path(order_id): Path<i64>,
    Json(req): Json<ScheduleRequest>,
) -> Result<Json<ScheduleResponse>, AppError> {
    let order = sqlx::query("SELECT id, winner_id, template_id, production_status FROM reward_order WHERE id = $1")
        .bind(order_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = order.ok_or_else(|| AppError::NotFound("Order not found".into()))?;
    let production_status: String = row.get("production_status");
    let template_id: i64 = row.get("template_id");

    if production_status != "pending" {
        return Err(AppError::BadRequest("Order is not in pending status".into()));
    }

    let store = sqlx::query("SELECT daily_capacity, status FROM store WHERE id = $1")
        .bind(req.store_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let store_row = store.ok_or_else(|| AppError::NotFound("Store not found".into()))?;
    let daily_capacity: i32 = store_row.get("daily_capacity");
    let store_status: String = store_row.get("status");
    if store_status != "active" {
        return Err(AppError::BadRequest("Store is not active".into()));
    }

    let scheduled_naive = req.scheduled_date.parse::<NaiveDateTime>()
        .map_err(|_| AppError::BadRequest("Invalid scheduled_date format, expected YYYY-MM-DDTHH:MM:SS".into()))?;
    let scheduled_dt = Utc.from_utc_datetime(&scheduled_naive);

    let existing_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM production_task WHERE store_id = $1 AND task_status IN ('pending', 'in_progress') AND created_at >= $2"
    )
    .bind(req.store_id)
    .bind(scheduled_dt)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    if existing_count >= daily_capacity as i64 {
        return Err(AppError::Conflict("Store capacity exceeded for the scheduled date".into()));
    }

    let activity_row = sqlx::query("SELECT activity_id FROM winner_record WHERE id = (SELECT winner_id FROM reward_order WHERE id = $1)")
        .bind(order_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let activity_id: i64 = activity_row.get("activity_id");

    let batch_row = sqlx::query(
        "INSERT INTO production_batch (store_id, activity_id, scheduled_date, total_count, status) VALUES ($1, $2, $3, 1, 'pending') RETURNING id"
    )
    .bind(req.store_id)
    .bind(activity_id)
    .bind(scheduled_dt)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let batch_id = batch_row.get::<i64, _>("id");

    let task_row = sqlx::query(
        "INSERT INTO production_task (batch_id, order_id, store_id, template_id, task_status) VALUES ($1, $2, $3, $4, 'pending') RETURNING id"
    )
    .bind(batch_id)
    .bind(order_id)
    .bind(req.store_id)
    .bind(template_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let task_id = task_row.get::<i64, _>("id");

    sqlx::query("UPDATE reward_order SET store_id = $1, scheduled_date = $2, production_status = 'scheduled' WHERE id = $3")
        .bind(req.store_id)
        .bind(scheduled_dt)
        .bind(order_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, 0, "order_scheduled", "reward_order", order_id, &format!("batch_id={}, store_id={}", batch_id, req.store_id)).await;

    Ok(Json(ScheduleResponse {
        batch_id,
        task_ids: vec![task_id],
    }))
}

#[derive(Serialize)]
pub struct ResendCodeResponse {
    pub new_code: String,
    pub order_id: i64,
}

pub async fn resend_code(
    State(state): State<AppState>,
    Path(order_id): Path<i64>,
) -> Result<Json<ResendCodeResponse>, AppError> {
    let order = sqlx::query("SELECT id, redeem_status FROM reward_order WHERE id = $1")
        .bind(order_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let _row = order.ok_or_else(|| AppError::NotFound("Order not found".into()))?;

    let old_codes = sqlx::query("SELECT id, code, status FROM redeem_code WHERE order_id = $1")
        .bind(order_id)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    for code_row in &old_codes {
        let code_id: i64 = code_row.get("id");
        let code_status: String = code_row.get("status");
        if code_status == "valid" || code_status == "expired" {
            sqlx::query("UPDATE redeem_code SET status = 'invalid' WHERE id = $1")
                .bind(code_id)
                .execute(&state.db_pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
        }
    }

    let new_code = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let expires_at = chrono::Utc::now() + chrono::Duration::days(7);

    sqlx::query("INSERT INTO redeem_code (order_id, code, expires_at, status) VALUES ($1, $2, $3, 'valid')")
        .bind(order_id)
        .bind(&new_code)
        .bind(expires_at)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("UPDATE reward_order SET redeem_status = 'pending' WHERE id = $1 AND redeem_status IN ('redeemed', 'expired')")
        .bind(order_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, 0, "resend_redeem_code", "reward_order", order_id, &format!("new_code={}", new_code)).await;

    Ok(Json(ResendCodeResponse { new_code, order_id }))
}

#[derive(Deserialize)]
pub struct CreatePaidOrderRequest {
    pub entry_id: i64,
    pub cake_size: String,
    pub cream_type: String,
    pub store_id: i64,
}

#[derive(Serialize)]
pub struct CreatePaidOrderResponse {
    pub order_id: i64,
    pub amount: f64,
    pub pay_status: String,
    pub prepay_id: Option<String>,
    pub prepay_params: Option<crate::services::wechat_pay::PrepayParams>,
}

pub async fn create_paid_order(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreatePaidOrderRequest>,
) -> Result<Json<CreatePaidOrderResponse>, AppError> {
    if req.entry_id <= 0 {
        return Err(AppError::BadRequest("entry_id is required".into()));
    }
    validation::validate_cake_size(&req.cake_size)?;
    validation::validate_cream_type(&req.cream_type)?;

    // Verify the entry exists, is active, and belongs to this user
    let entry = sqlx::query("SELECT id, activity_id, user_id, status FROM contest_entry WHERE id = $1")
        .bind(req.entry_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let entry_row = entry.ok_or_else(|| AppError::NotFound("Entry not found".into()))?;
    let entry_user_id: i64 = entry_row.get("user_id");
    let activity_id: i64 = entry_row.get("activity_id");
    let entry_status: String = entry_row.get("status");
    if entry_user_id != claims.user_id {
        return Err(AppError::Forbidden("Not your entry".into()));
    }
    if entry_status != "active" {
        return Err(AppError::BadRequest(format!("Cannot order for entry with status '{}'", entry_status)));
    }

    // Get region_id from activity, and verify activity status allows ordering
    let activity = sqlx::query("SELECT region_id, status FROM activity WHERE id = $1")
        .bind(activity_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let region_id: i64 = activity.get("region_id");
    let activity_status: String = activity.get("status");
    if !matches!(activity_status.as_str(), "redeeming" | "finished" | "settled" | "voting_closed") {
        return Err(AppError::BadRequest(format!("Cannot place order while activity is '{}'", activity_status)));
    }

    // Look up price from price_config
    let price_row = sqlx::query(
        "SELECT price FROM price_config WHERE region_id = $1 AND cake_size = $2 AND cream_type = $3 AND status = 'active'"
    )
    .bind(region_id)
    .bind(&req.cake_size)
    .bind(&req.cream_type)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    let price_row = price_row.ok_or_else(|| {
        AppError::NotFound(format!("No price configured for {} {}/{}", req.cake_size, req.cream_type, region_id))
    })?;
    let amount: f64 = price_row.get("price");

    // Verify store exists, is active, and belongs to the same region
    let store = sqlx::query("SELECT id, status, region_id FROM store WHERE id = $1")
        .bind(req.store_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let store_row = store.ok_or_else(|| AppError::NotFound("Store not found".into()))?;
    let store_status: String = store_row.get("status");
    if store_status != "active" {
        return Err(AppError::BadRequest("Store is not active".into()));
    }
    let store_region_id: i64 = store_row.get("region_id");
    if store_region_id != region_id {
        return Err(AppError::BadRequest("Store does not belong to the activity's region".into()));
    }

    // Idempotency + order creation in a single transaction to prevent race conditions
    let mut tx = state.db_pool.begin().await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let existing: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM reward_order WHERE entry_id = $1 AND pay_status IN ('pending', 'paid') LIMIT 1 FOR UPDATE"
    )
    .bind(req.entry_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    if let Some(existing_id) = existing {
        tx.rollback().await.map_err(|e| AppError::Internal(e.to_string()))?;
        return Err(AppError::Conflict(format!("Order already exists for entry_id {}: order_id {}", req.entry_id, existing_id)));
    }

    let order_row = sqlx::query(
        "INSERT INTO reward_order (winner_id, store_id, production_status, redeem_status, order_type, amount, pay_status, template_id, entry_id, user_id) VALUES (0, $1, 'pending', 'pending', 'paid', $2, 'pending', 0, $3, $4) RETURNING id"
    )
    .bind(req.store_id)
    .bind(amount)
    .bind(req.entry_id)
    .bind(claims.user_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let order_id: i64 = order_row.get("id");

    // Generate redeem code for self-pickup
    let code = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let expires_at = chrono::Utc::now() + chrono::Duration::days(7);
    sqlx::query("INSERT INTO redeem_code (order_id, code, expires_at, status) VALUES ($1, $2, $3, 'valid')")
        .bind(order_id)
        .bind(&code)
        .bind(expires_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    tx.commit().await
        .map_err(|e| AppError::Internal(format!("Transaction commit failed: {}", e)))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "create_paid_order", "reward_order", order_id,
        &format!("entry_id={}, amount={}", req.entry_id, amount)).await;

    // Attempt to create WeChat JSAPI prepay order
    let amount_fen = (amount * 100.0).round() as i64;
    let description = format!("{} {} 蛋糕", req.cake_size, req.cream_type);

    // Fetch user's open_id for WeChat JSAPI payment
    let open_id: String = sqlx::query_scalar("SELECT COALESCE(open_id, '') FROM app_user WHERE id = $1")
        .bind(claims.user_id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap_or_default();

    let jsapi_result = crate::services::wechat_pay::create_jsapi_order(
        &state.http_client, &state.config, order_id, amount_fen, &description, &open_id,
    ).await;

    let (prepay_id, prepay_params) = match jsapi_result {
        Ok(r) => (Some(r.prepay_id), r.prepay_params),
        Err(e) => {
            tracing::warn!("JSAPI order creation failed (order {}): {} — client will need to retry payment", order_id, e);
            (None, None)
        }
    };

    Ok(Json(CreatePaidOrderResponse {
        order_id,
        amount,
        pay_status: "pending".to_string(),
        prepay_id,
        prepay_params,
    }))
}

#[derive(Deserialize)]
pub struct RefundRequest {
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct RefundResponse {
    pub order_id: i64,
    pub refund_status: String,
}

pub async fn refund(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(order_id): Path<i64>,
    Json(req): Json<RefundRequest>,
) -> Result<Json<RefundResponse>, AppError> {
    let order = sqlx::query(
        "SELECT id, order_type, pay_status, refund_status FROM reward_order WHERE id = $1"
    )
    .bind(order_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = order.ok_or_else(|| AppError::NotFound("Order not found".into()))?;
    let order_type: String = row.get("order_type");
    let pay_status: String = row.get("pay_status");
    let refund_status: Option<String> = row.get("refund_status");

    if order_type != "paid" {
        return Err(AppError::BadRequest("Only paid orders can be refunded".into()));
    }
    if pay_status != "paid" {
        return Err(AppError::BadRequest("Order is not in paid status".into()));
    }
    if refund_status.as_deref() == Some("refunded") {
        return Err(AppError::Conflict("Order already refunded".into()));
    }

    // Call WeChat Refund API (returns stub if credentials not configured)
    let amount_fen = (row.get::<f64, _>("amount") * 100.0) as i64;
    let refund_txn_id = wechat_pay::submit_refund(
        &state.http_client,
        &state.config,
        &format!("ord_{}", order_id),
        amount_fen,
        amount_fen,
    ).await?;

    sqlx::query(
        "UPDATE reward_order SET refund_status = 'refunded', refund_reason = $1, refund_txn_id = $2, refunded_at = NOW() WHERE id = $3"
    )
    .bind(req.reason.as_deref().unwrap_or("Admin initiated refund"))
    .bind(&refund_txn_id)
    .bind(order_id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    // Insert payment audit record
    sqlx::query(
        "INSERT INTO payment_record (order_id, transaction_id, pay_channel, amount, status, raw_response) SELECT $1, $2, 'refund', amount, 'refunded', '{}'::jsonb FROM reward_order WHERE id = $1"
    )
    .bind(order_id)
    .bind(&refund_txn_id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    // Invalidate any valid redeem codes for this order
    sqlx::query("UPDATE redeem_code SET status = 'invalid' WHERE order_id = $1 AND status = 'valid'")
        .bind(order_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "refund_order", "reward_order", order_id,
        &format!("refund_txn_id={}", refund_txn_id)).await;

    Ok(Json(RefundResponse {
        order_id,
        refund_status: "refunded".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct PayCallbackRequest {
    pub transaction_id: Option<String>,
    pub pay_channel: Option<String>,
    pub raw_response: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct PayCallbackResponse {
    pub order_id: i64,
    pub pay_status: String,
}

pub async fn pay_callback(
    State(state): State<AppState>,
    Path(order_id): Path<i64>,
    headers: HeaderMap,
    body: String,
) -> Result<Json<PayCallbackResponse>, AppError> {
    // Verify WeChat Pay signature
    wechat_pay::verify_if_configured(&headers, &body, &state.config.wechat_pay_platform_cert)?;

    // Parse the JSON body after signature verification
    let req: PayCallbackRequest = serde_json::from_str(&body)
        .map_err(|e| AppError::BadRequest(format!("Invalid callback body: {}", e)))?;

    // Use transaction + FOR UPDATE to prevent concurrent callback race
    let mut tx = state.db_pool.begin().await.map_err(|e| AppError::Internal(e.to_string()))?;

    let order = sqlx::query(
        "SELECT id, pay_status FROM reward_order WHERE id = $1 FOR UPDATE"
    )
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = order.ok_or_else(|| AppError::NotFound("Order not found".into()))?;
    let pay_status: String = row.get("pay_status");

    if pay_status == "paid" {
        tx.rollback().await.map_err(|e| AppError::Internal(e.to_string()))?;
        return Err(AppError::Conflict("Order already paid".into()));
    }
    if pay_status == "closed" {
        tx.rollback().await.map_err(|e| AppError::Internal(e.to_string()))?;
        return Err(AppError::BadRequest("Order is closed".into()));
    }

    let txn_id = req.transaction_id.unwrap_or_else(|| format!("stub_pay_{}", &uuid::Uuid::new_v4().to_string()[..8]));
    let channel = req.pay_channel.unwrap_or_else(|| "wechat_h5".into());
    let raw = req.raw_response.unwrap_or(serde_json::json!({}));

    // Conditional UPDATE — only sets paid if still pending (idempotent guard)
    let result = sqlx::query(
        "UPDATE reward_order SET pay_status = 'paid', pay_transaction_id = $1, paid_at = NOW() WHERE id = $2 AND pay_status = 'pending'"
    )
    .bind(&txn_id)
    .bind(order_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    if result.rows_affected() == 0 {
        tx.rollback().await.map_err(|e| AppError::Internal(e.to_string()))?;
        return Err(AppError::Conflict("Order already processed".into()));
    }

    sqlx::query(
        "INSERT INTO payment_record (order_id, transaction_id, pay_channel, amount, status, raw_response) \
         SELECT $1, $2, $3, amount, 'success', $4 FROM reward_order WHERE id = $1"
    )
    .bind(order_id)
    .bind(&txn_id)
    .bind(&channel)
    .bind(&raw)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    tx.commit().await.map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, 0, "pay_callback", "reward_order", order_id,
        &format!("txn_id={}, channel={}", txn_id, channel)).await;

    Ok(Json(PayCallbackResponse {
        order_id,
        pay_status: "paid".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct OrderListQuery {
    pub pay_status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct OrderListResponse {
    pub list: Vec<OrderRow>,
    pub total: i64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct OrderRow {
    pub id: i64,
    pub winner_id: i64,
    pub store_id: i64,
    pub template_id: i64,
    pub order_type: String,
    pub amount: f64,
    pub pay_status: String,
    pub production_status: String,
    pub redeem_status: String,
    pub refund_status: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<OrderListQuery>,
) -> Result<Json<OrderListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let mut qb = sqlx::QueryBuilder::new(
        "SELECT id, winner_id, store_id, template_id, order_type, COALESCE(amount, 0) as amount, \
         COALESCE(pay_status, 'free') as pay_status, production_status, redeem_status, refund_status, created_at \
         FROM reward_order WHERE 1=1"
    );
    if let Some(ref ps) = params.pay_status {
        qb.push(" AND pay_status = ");
        qb.push_bind(ps);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(page_size);
    qb.push(" OFFSET ");
    qb.push_bind(offset);

    let orders = qb
        .build_query_as::<OrderRow>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut count_qb = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM reward_order WHERE 1=1");
    if let Some(ref ps) = params.pay_status {
        count_qb.push(" AND pay_status = ");
        count_qb.push_bind(ps);
    }

    let total: i64 = count_qb
        .build_query_scalar()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(OrderListResponse { list: orders, total }))
}

#[derive(Serialize)]
pub struct OrderDetailResponse {
    pub id: i64,
    pub winner_id: i64,
    pub store_id: i64,
    pub template_id: i64,
    pub order_type: String,
    pub amount: f64,
    pub pay_status: String,
    pub production_status: String,
    pub redeem_status: String,
    pub refund_status: Option<String>,
    pub refund_reason: Option<String>,
    pub redeem_code: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn detail(
    State(state): State<AppState>,
    Path(order_id): Path<i64>,
) -> Result<Json<OrderDetailResponse>, AppError> {
    let row = sqlx::query(
        "SELECT ro.id, ro.winner_id, ro.store_id, ro.template_id, \
         COALESCE(ro.order_type, 'free') as order_type, \
         COALESCE(ro.amount, 0) as amount, \
         COALESCE(ro.pay_status, 'free') as pay_status, \
         ro.production_status, ro.redeem_status, \
         ro.refund_status, ro.refund_reason, \
         rc.code as redeem_code, \
         ro.created_at, ro.paid_at \
         FROM reward_order ro \
         LEFT JOIN LATERAL (SELECT code FROM redeem_code WHERE order_id = ro.id AND status = 'valid' LIMIT 1) rc ON TRUE \
         WHERE ro.id = $1"
    )
    .bind(order_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .ok_or_else(|| AppError::NotFound("Order not found".into()))?;

    Ok(Json(OrderDetailResponse {
        id: row.get("id"),
        winner_id: row.get("winner_id"),
        store_id: row.get("store_id"),
        template_id: row.get("template_id"),
        order_type: row.get("order_type"),
        amount: row.get("amount"),
        pay_status: row.get("pay_status"),
        production_status: row.get("production_status"),
        redeem_status: row.get("redeem_status"),
        refund_status: row.get("refund_status"),
        refund_reason: row.get("refund_reason"),
        redeem_code: row.get("redeem_code"),
        created_at: row.get("created_at"),
        paid_at: row.get("paid_at"),
    }))
}

#[derive(Serialize)]
pub struct InitPayResponse {
    pub order_id: i64,
    pub prepay_id: String,
    pub prepay_params: Option<crate::services::wechat_pay::PrepayParams>,
}

pub async fn init_pay(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(order_id): Path<i64>,
) -> Result<Json<InitPayResponse>, AppError> {
    let order = sqlx::query(
        "SELECT id, order_type, pay_status, amount FROM reward_order WHERE id = $1 AND user_id = $2"
    )
    .bind(order_id)
    .bind(claims.user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let row = order.ok_or_else(|| AppError::NotFound("Order not found or not yours".into()))?;
    let order_type: String = row.get("order_type");
    let pay_status: String = row.get("pay_status");

    if order_type != "paid" {
        return Err(AppError::BadRequest("Only paid-type orders can initiate payment".into()));
    }
    if pay_status != "pending" {
        return Err(AppError::BadRequest(format!("Order pay_status is {}, cannot re-initiate", pay_status)));
    }

    let amount: f64 = row.get("amount");
    let amount_fen = (amount * 100.0).round() as i64;
    let description = format!("FreeCake order #{}", order_id);
    let open_id = claims.open_id.as_deref().unwrap_or("");

    let result = crate::services::wechat_pay::create_jsapi_order(
        &state.http_client, &state.config, order_id, amount_fen, &description, open_id,
    ).await?;

    Ok(Json(InitPayResponse {
        order_id,
        prepay_id: result.prepay_id,
        prepay_params: result.prepay_params,
    }))
}

#[derive(Deserialize)]
pub struct CancelOrderRequest {
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct CancelOrderResponse {
    pub order_id: i64,
    pub pay_status: String,
}

pub async fn cancel(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(order_id): Path<i64>,
    Json(req): Json<CancelOrderRequest>,
) -> Result<Json<CancelOrderResponse>, AppError> {
    let order = sqlx::query(
        "SELECT id, pay_status, order_type FROM reward_order WHERE id = $1 AND user_id = $2"
    )
    .bind(order_id)
    .bind(claims.user_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let row = order.ok_or_else(|| AppError::NotFound("Order not found or not yours".into()))?;
    let pay_status: String = row.get("pay_status");

    if pay_status != "pending" {
        return Err(AppError::BadRequest(format!("Cannot cancel order with pay_status {}", pay_status)));
    }

    sqlx::query(
        "UPDATE reward_order SET pay_status = 'closed', closed_at = NOW() WHERE id = $1 AND pay_status = 'pending'"
    )
    .bind(order_id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("UPDATE redeem_code SET status = 'invalid' WHERE order_id = $1 AND status = 'valid'")
        .bind(order_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "cancel_order", "reward_order", order_id,
        req.reason.as_deref().unwrap_or("user cancelled")).await;

    Ok(Json(CancelOrderResponse {
        order_id,
        pay_status: "closed".to_string(),
    }))
}
