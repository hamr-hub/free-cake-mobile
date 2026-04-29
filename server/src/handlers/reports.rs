use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct ReportsQuery {
    pub start: Option<String>,
    pub end: Option<String>,
    pub region_id: Option<i64>,
}
#[derive(Serialize)]
pub struct ReportsSummaryResponse {
    pub total_entries: i64,
    pub total_votes: i64,
    pub redeem_rate: f64,
    pub conversion_rate: f64,
    pub entries: Vec<DateCount>,
    pub votes: Vec<DateCount>,
    pub regions: Vec<RegionBrief>,
}

#[derive(Serialize)]
pub struct DateCount {
    pub date: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct RegionBrief {
    pub id: i64,
    pub name: String,
}

pub async fn summary(
    State(state): State<AppState>,
    Query(params): Query<ReportsQuery>,
) -> Result<Json<ReportsSummaryResponse>, AppError> {
    let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT DATE(created_at) AS date, COUNT(*) AS count FROM contest_entry WHERE 1=1"
    );
    let mut builder_v = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT DATE(created_at) AS date, COUNT(*) AS count FROM vote_record WHERE 1=1"
    );

    if let Some(ref start) = params.start {
        builder.push(" AND created_at >= "); builder.push_bind(start);
        builder_v.push(" AND created_at >= "); builder_v.push_bind(start);
    }
    if let Some(ref end) = params.end {
        builder.push(" AND created_at <= "); builder.push_bind(end);
        builder_v.push(" AND created_at <= "); builder_v.push_bind(end);
    }
    if let Some(region_id) = params.region_id {
        builder.push(" AND activity_id IN (SELECT id FROM activity WHERE region_id = "); builder.push_bind(region_id); builder.push(")");
        builder_v.push(" AND activity_id IN (SELECT id FROM activity WHERE region_id = "); builder_v.push_bind(region_id); builder_v.push(")");
    }

    builder.push(" GROUP BY DATE(created_at) ORDER BY date");
    builder_v.push(" GROUP BY DATE(created_at) ORDER BY date");

    let entry_rows = builder.build().fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    let vote_rows = builder_v.build().fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let entries: Vec<DateCount> = entry_rows.iter().map(|r| DateCount {
        date: r.get::<chrono::NaiveDate, _>("date").to_string(),
        count: r.get("count"),
    }).collect();
    let votes: Vec<DateCount> = vote_rows.iter().map(|r| DateCount {
        date: r.get::<chrono::NaiveDate, _>("date").to_string(),
        count: r.get("count"),
    }).collect();

    let total_entries: i64 = entries.iter().map(|e| e.count).sum();
    let total_votes: i64 = votes.iter().map(|v| v.count).sum();

    let mut redeem_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT COUNT(*) AS redeemed FROM redeem_record WHERE 1=1"
    );
    let mut order_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT COUNT(*) AS total FROM reward_order WHERE 1=1"
    );
    let mut conversion_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "SELECT COUNT(*) FROM reward_order WHERE redeem_status = 'pending'"
    );

    if let Some(ref start) = params.start {
        redeem_builder.push(" AND created_at >= "); redeem_builder.push_bind(start);
        order_builder.push(" AND created_at >= "); order_builder.push_bind(start);
        conversion_builder.push(" AND created_at >= "); conversion_builder.push_bind(start);
    }
    if let Some(ref end) = params.end {
        redeem_builder.push(" AND created_at <= "); redeem_builder.push_bind(end);
        order_builder.push(" AND created_at <= "); order_builder.push_bind(end);
        conversion_builder.push(" AND created_at <= "); conversion_builder.push_bind(end);
    }
    if let Some(region_id) = params.region_id {
        redeem_builder.push(" AND store_id IN (SELECT id FROM store WHERE region_id = "); redeem_builder.push_bind(region_id); redeem_builder.push(")");
        order_builder.push(" AND winner_id IN (SELECT id FROM winner_record WHERE activity_id IN (SELECT id FROM activity WHERE region_id = "); order_builder.push_bind(region_id); order_builder.push("))");
        conversion_builder.push(" AND winner_id IN (SELECT id FROM winner_record WHERE activity_id IN (SELECT id FROM activity WHERE region_id = "); conversion_builder.push_bind(region_id); conversion_builder.push("))");
    }

    let redeem_row = redeem_builder.build().fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    let order_row = order_builder.build().fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    let redeemed: i64 = redeem_row.get("redeemed");
    let total_orders: i64 = order_row.get("total");
    let redeem_rate = if total_orders > 0 { redeemed as f64 / total_orders as f64 * 100.0 } else { 0.0 };

    let conversion_rate = if total_entries > 0 {
        let paid: i64 = conversion_builder.build_query_scalar::<i64>()
            .fetch_one(&state.db_pool).await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        paid as f64 / total_entries as f64 * 100.0
    } else { 0.0 };

    let region_rows = sqlx::query("SELECT id, name FROM region WHERE status = 'active' ORDER BY name")
        .fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    let regions: Vec<RegionBrief> = region_rows.iter().map(|r| RegionBrief {
        id: r.get("id"),
        name: r.get("name"),
    }).collect();

    Ok(Json(ReportsSummaryResponse {
        total_entries,
        total_votes,
        redeem_rate,
        conversion_rate,
        entries,
        votes,
        regions,
    }))
}

#[derive(Deserialize)]
pub struct PeriodReportQuery {
    pub date: Option<String>,
    pub region_id: Option<i64>,
}

#[derive(Serialize)]
pub struct PeriodReportResponse {
    pub period: String,
    pub total_orders: i64,
    pub total_paid_orders: i64,
    pub total_revenue: f64,
    pub total_refunded: f64,
    pub total_entries: i64,
    pub total_votes: i64,
    pub top_stores: Vec<StoreRevenue>,
}

#[derive(Serialize)]
pub struct StoreRevenue {
    pub store_id: i64,
    pub store_name: String,
    pub order_count: i64,
    pub revenue: f64,
}

fn apply_region_filter(q: &mut String, region_id: Option<i64>, alias: &str) -> Option<i64> {
    if let Some(rid) = region_id {
        match alias {
            "reward_order" => q.push_str(&format!(" AND winner_id IN (SELECT id FROM winner_record WHERE activity_id IN (SELECT id FROM activity WHERE region_id = {}))", rid)),
            "contest_entry" | "vote_record" => q.push_str(&format!(" AND activity_id IN (SELECT id FROM activity WHERE region_id = {})", rid)),
            "redeem_record" => q.push_str(&format!(" AND store_id IN (SELECT id FROM store WHERE region_id = {})", rid)),
            "store" => q.push_str(&format!(" AND s.region_id = {}", rid)),
            _ => {}
        }
        Some(rid)
    } else {
        None
    }
}

pub async fn daily_report(
    State(state): State<AppState>,
    Query(params): Query<PeriodReportQuery>,
) -> Result<Json<PeriodReportResponse>, AppError> {
    let date = params.date.unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string());
    let start = format!("{} 00:00:00", date);
    let end = format!("{} 23:59:59", date);

    let mut q = "SELECT COUNT(*) FROM reward_order WHERE created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_orders: i64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COUNT(*) FROM reward_order WHERE pay_status = 'paid' AND created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_paid_orders: i64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COALESCE(SUM(amount), 0) FROM reward_order WHERE pay_status = 'paid' AND created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_revenue: f64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COALESCE(SUM(amount), 0) FROM reward_order WHERE refund_status = 'refunded' AND created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_refunded: f64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COUNT(*) FROM contest_entry WHERE created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "contest_entry");
    let total_entries: i64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COUNT(*) FROM vote_record WHERE created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "vote_record");
    let total_votes: i64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT s.id as store_id, s.name as store_name, COUNT(ro.id) as order_count, COALESCE(SUM(ro.amount), 0) as revenue \
         FROM store s LEFT JOIN reward_order ro ON ro.store_id = s.id AND ro.pay_status = 'paid' AND ro.created_at >= $1 AND ro.created_at <= $2 \
         WHERE 1=1".to_string();
    apply_region_filter(&mut q, params.region_id, "store");
    q.push_str(" GROUP BY s.id, s.name ORDER BY revenue DESC LIMIT 10");
    let store_rows = sqlx::query(&q).bind(&start).bind(&end).fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let top_stores: Vec<StoreRevenue> = store_rows.iter().map(|r| StoreRevenue {
        store_id: r.get("store_id"),
        store_name: r.get("store_name"),
        order_count: r.get("order_count"),
        revenue: r.get("revenue"),
    }).collect();

    Ok(Json(PeriodReportResponse {
        period: date,
        total_orders,
        total_paid_orders,
        total_revenue,
        total_refunded,
        total_entries,
        total_votes,
        top_stores,
    }))
}

pub async fn weekly_report(
    State(state): State<AppState>,
    Query(params): Query<PeriodReportQuery>,
) -> Result<Json<PeriodReportResponse>, AppError> {
    let date = params.date.unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string());
    let end_date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Invalid date format".into()))?;
    let start_date = end_date - chrono::Duration::days(6);
    let start = format!("{} 00:00:00", start_date);
    let end = format!("{} 23:59:59", end_date);
    let period_label = format!("{} ~ {}", start_date, end_date);

    let mut q = "SELECT COUNT(*) FROM reward_order WHERE created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_orders: i64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COUNT(*) FROM reward_order WHERE pay_status = 'paid' AND created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_paid_orders: i64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COALESCE(SUM(amount), 0) FROM reward_order WHERE pay_status = 'paid' AND created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_revenue: f64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COALESCE(SUM(amount), 0) FROM reward_order WHERE refund_status = 'refunded' AND created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_refunded: f64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COUNT(*) FROM contest_entry WHERE created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "contest_entry");
    let total_entries: i64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COUNT(*) FROM vote_record WHERE created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "vote_record");
    let total_votes: i64 = sqlx::query_scalar(&q).bind(&start).bind(&end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT s.id as store_id, s.name as store_name, COUNT(ro.id) as order_count, COALESCE(SUM(ro.amount), 0) as revenue \
         FROM store s LEFT JOIN reward_order ro ON ro.store_id = s.id AND ro.pay_status = 'paid' AND ro.created_at >= $1 AND ro.created_at <= $2 \
         WHERE 1=1".to_string();
    apply_region_filter(&mut q, params.region_id, "store");
    q.push_str(" GROUP BY s.id, s.name ORDER BY revenue DESC LIMIT 10");
    let store_rows = sqlx::query(&q).bind(&start).bind(&end).fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let top_stores: Vec<StoreRevenue> = store_rows.iter().map(|r| StoreRevenue {
        store_id: r.get("store_id"),
        store_name: r.get("store_name"),
        order_count: r.get("order_count"),
        revenue: r.get("revenue"),
    }).collect();

    Ok(Json(PeriodReportResponse {
        period: period_label,
        total_orders,
        total_paid_orders,
        total_revenue,
        total_refunded,
        total_entries,
        total_votes,
        top_stores,
    }))
}

pub async fn monthly_report(
    State(state): State<AppState>,
    Query(params): Query<PeriodReportQuery>,
) -> Result<Json<PeriodReportResponse>, AppError> {
    let date = params.date.unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string());
    let month_start = format!("{}-01 00:00:00", &date[..7]);
    let end_date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Invalid date format".into()))?;
    let last_day = (end_date + chrono::Duration::days(32)).format("%Y-%m-01").to_string();
    let last_day_date = chrono::NaiveDate::parse_from_str(&last_day, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Invalid end date".into()))?;
    let month_end_date = last_day_date - chrono::Duration::days(1);
    let month_end = format!("{} 23:59:59", month_end_date.format("%Y-%m-%d"));

    let mut q = "SELECT COUNT(*) FROM reward_order WHERE created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_orders: i64 = sqlx::query_scalar(&q).bind(&month_start).bind(&month_end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COUNT(*) FROM reward_order WHERE pay_status = 'paid' AND created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_paid_orders: i64 = sqlx::query_scalar(&q).bind(&month_start).bind(&month_end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COALESCE(SUM(amount), 0) FROM reward_order WHERE pay_status = 'paid' AND created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_revenue: f64 = sqlx::query_scalar(&q).bind(&month_start).bind(&month_end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COALESCE(SUM(amount), 0) FROM reward_order WHERE refund_status = 'refunded' AND created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "reward_order");
    let total_refunded: f64 = sqlx::query_scalar(&q).bind(&month_start).bind(&month_end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COUNT(*) FROM contest_entry WHERE created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "contest_entry");
    let total_entries: i64 = sqlx::query_scalar(&q).bind(&month_start).bind(&month_end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT COUNT(*) FROM vote_record WHERE created_at >= $1 AND created_at <= $2".to_string();
    apply_region_filter(&mut q, params.region_id, "vote_record");
    let total_votes: i64 = sqlx::query_scalar(&q).bind(&month_start).bind(&month_end).fetch_one(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let mut q = "SELECT s.id as store_id, s.name as store_name, COUNT(ro.id) as order_count, COALESCE(SUM(ro.amount), 0) as revenue \
         FROM store s LEFT JOIN reward_order ro ON ro.store_id = s.id AND ro.pay_status = 'paid' AND ro.created_at >= $1 AND ro.created_at <= $2 \
         WHERE 1=1".to_string();
    apply_region_filter(&mut q, params.region_id, "store");
    q.push_str(" GROUP BY s.id, s.name ORDER BY revenue DESC LIMIT 10");
    let store_rows = sqlx::query(&q).bind(&month_start).bind(&month_end).fetch_all(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    let top_stores: Vec<StoreRevenue> = store_rows.iter().map(|r| StoreRevenue {
        store_id: r.get("store_id"),
        store_name: r.get("store_name"),
        order_count: r.get("order_count"),
        revenue: r.get("revenue"),
    }).collect();

    Ok(Json(PeriodReportResponse {
        period: date[..7].to_string(),
        total_orders,
        total_paid_orders,
        total_revenue,
        total_refunded,
        total_entries,
        total_votes,
        top_stores,
    }))
}

pub async fn reconciliation(
    State(state): State<AppState>,
) -> Result<Json<crate::services::reconciliation::ReconciliationResult>, AppError> {
    let result = crate::services::reconciliation::reconcile(&state.db_pool).await?;
    Ok(Json(result))
}
