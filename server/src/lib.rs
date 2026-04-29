pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod app_middleware;
pub mod services;

use axum::{Router, Json, extract::State, middleware, routing::{get, post, patch, put}};
use sqlx::PgPool;
use tower_http::cors::{CorsLayer, Any};
use axum::http::HeaderValue;
use metrics_exporter_prometheus::PrometheusBuilder;
use serde_json::json;

use handlers::{auth, activity, entry, vote, settlement, order, redeem, inventory, store, dashboard, region, production, staff_handler, query_handlers, user, upload, reports, template, price};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_client: redis::Client,
    pub config: config::AppConfig,
    pub sms_service: services::sms::SmsService,
    pub crypto_service: services::crypto::CryptoService,
    pub http_client: reqwest::Client,
}

pub fn create_router(state: AppState) -> Router {
    // Install Prometheus recorder (idempotent — no-op if already installed)
    let recorder = PrometheusBuilder::new().install_recorder().expect("Failed to install Prometheus recorder");

    let cors = if state.config.cors_origin.is_empty() {
        tracing::warn!("CORS_ORIGIN is empty — allowing all origins. Do NOT use in production.");
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        let origins: Vec<HeaderValue> = state.config.cors_origin.split(',')
            .filter_map(|o| o.trim().parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
            .allow_headers(Any)
    };

    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(move || std::future::ready(recorder.render())))
        .route("/auth/login", post(auth::login))
        .route("/auth/wechat-login", post(auth::wechat_login))
        .route("/auth/bind-phone", post(auth::bind_phone))
        .route("/auth/send-verify-code", post(query_handlers::send_verify_code))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::public_rate_limit::public_rate_limit_middleware));

    let admin_routes = Router::new()
        .route("/activities", post(activity::create))
        .route("/activities/:id", put(activity::update))
        .route("/activities/:id/status", post(activity::update_status))
        .route("/activities/:id/rules", put(activity::update_rules))
        .route("/activities/:id/settle", post(settlement::settle))
        .route("/entries", get(query_handlers::entry_list))
        .route("/entries/:id/status", post(entry::update_status))
        .route("/entries/:id/freeze", post(entry::freeze))
        .route("/entries/:id/deduct", post(entry::deduct_votes))
        .route("/votes/risk", get(query_handlers::vote_risk_list))
        .route("/settlement", get(query_handlers::winner_list))
        .route("/settlement/:id", get(query_handlers::show_winner))
        .route("/production", get(query_handlers::production_list))
        .route("/redeem", get(query_handlers::redeem_list))
        .route("/audit_log", get(query_handlers::list))
        .route("/audit_log/:id", get(query_handlers::show_audit_log))
        .route("/audit-log", get(query_handlers::list))
        .route("/audit-log/:id", get(query_handlers::show_audit_log))
        .route("/risk_events", get(query_handlers::risk_event_list))
        .route("/risk_events/:id", get(query_handlers::risk_event_show))
        .route("/risk-events", get(query_handlers::risk_event_list))
        .route("/risk-events/:id", get(query_handlers::risk_event_show))
        .route("/orders/:id/schedule", post(order::schedule))
        .route("/orders/:id/resend-code", post(order::resend_code))
        .route("/orders/:id/refund", post(order::refund))
        .route("/orders/:id/pay-callback", put(order::pay_callback))
        .route("/redeem/verify", post(redeem::verify))
        .route("/stores/:id/inventory", get(inventory::get_by_store))
        .route("/inventory", post(inventory::create_item).get(inventory::list))
        .route("/inventory_txn", post(inventory::create_txn))
        .route("/inventory/:id", patch(inventory::update_item))
        .route("/inventory/items/:id", get(inventory::show_item))
        .route("/upload", post(upload::upload))
        .route("/templates", post(template::create))
        .route("/templates/:id", put(template::update))
        .route("/templates/:id/status", post(template::update_status))
        .route("/prices", post(price::create))
        .route("/prices/:id", put(price::update).get(price::show))
        .route("/reports/summary", get(reports::summary))
        .route("/reports/reconciliation", get(reports::reconciliation))
        .route("/reports/daily", get(reports::daily_report))
        .route("/reports/weekly", get(reports::weekly_report))
        .route("/reports/monthly", get(reports::monthly_report))
        .route("/stores", post(store::create))
        .route("/stores/:id", put(store::update))
        .route("/stores/:id/status", post(store::update_status))
        .route("/regions", post(region::create))
        .route("/regions/:id", put(region::update))
        .route("/regions/:id/status", post(region::update_status))
        .route("/production/tasks/:id/start", post(production::start_task))
        .route("/production/tasks/:id/pause", post(production::pause_task))
        .route("/production/tasks/:id/resume", post(production::resume_task))
        .route("/production/tasks/:id/complete", post(production::complete_task))
        .route("/production/tasks/:id/error", post(production::report_error))
        .route("/production/tasks/:id/cancel", post(production::cancel_task))
        .route("/staff", post(staff_handler::create).get(staff_handler::list))
        .route("/staff/:id", get(staff_handler::show).put(staff_handler::update))
        .route("/staff/:id/status", post(staff_handler::update_status))
        .route("/staff/check-in", post(staff_handler::check_in))
        .route("/staff/check-out", post(staff_handler::check_out))
        .layer(middleware::from_fn(app_middleware::auth::admin_only_middleware))
        .layer(middleware::from_fn(app_middleware::error_handler::error_handler_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::auth::auth_middleware));

    let user_routes = Router::new()
        .route("/dashboard/stats", get(dashboard::stats))
        .route("/activities", get(activity::list))
        .route("/activities/:id", get(activity::show))
        .route("/activities/:id/rules", get(activity::get_rules))
        .route("/activities/:id/rank", get(vote::rank))
        .route("/activities/:id/entries/generate", post(entry::generate))
        .route("/activities/:id/entries", post(entry::submit))
        .route("/entries/:id", get(entry::show))
        .route("/entries/:id/vote", post(vote::cast))
        .route("/entries/mine", get(user::my_entries))
        .route("/votes/mine", get(user::my_votes))
        .route("/orders", get(order::list).post(order::create_paid_order))
        .route("/orders/:id", get(order::detail))
        .route("/orders/:id/init-pay", post(order::init_pay))
        .route("/orders/:id/cancel", post(order::cancel))
        .route("/orders/mine", get(user::my_orders))
        .route("/users/me", get(user::me).put(user::update_me))
        .route("/users/resolve-region", get(user::resolve_region))
        .route("/redeem/:code", get(user::redeem_detail))
        .route("/auth/refresh", post(auth::refresh))
        .route("/auth/logout", post(auth::logout))
        .route("/stores", get(store::list))
        .route("/stores/:id", get(store::show))
        .route("/regions", get(region::list))
        .route("/regions/:id", get(region::show))
        .route("/templates", get(template::list))
        .route("/templates/:id", get(template::show))
        .route("/activities/templates", get(template::list))
        .route("/prices", get(price::list))
        .route("/inventory", get(inventory::list))
        .route("/attendance", get(staff_handler::list_attendance))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::rate_limit::ip_rate_limit_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::auth::auth_middleware))
        .layer(middleware::from_fn(app_middleware::error_handler::error_handler_middleware));

    Router::new()
        .nest("/api", public_routes.merge(admin_routes).merge(user_routes))
        .layer(cors)
        .layer(middleware::from_fn(app_middleware::metrics::metrics_middleware))
        .layer(middleware::from_fn(app_middleware::request_id::request_id_middleware))
        .layer(middleware::from_fn(app_middleware::security_headers::security_headers_middleware))
        .layer(middleware::from_fn(app_middleware::limit_body::limit_body_middleware))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}

async fn health_check(State(state): State<AppState>) -> axum::response::Response<axum::body::Body> {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let mut details = serde_json::Map::new();
    let mut healthy = true;

    // Check PostgreSQL
    match sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(&state.db_pool).await {
        Ok(_) => { details.insert("postgres".into(), json!("ok")); }
        Err(e) => {
            tracing::error!("Health check: PostgreSQL down: {}", e);
            details.insert("postgres".into(), json!(format!("error: {}", e)));
            healthy = false;
        }
    }

    // Check Redis
    let redis_healthy = match state.redis_client.get_multiplexed_async_connection().await {
        Ok(mut conn) => match redis::cmd("PING").query_async::<String>(&mut conn).await {
            Ok(_) => { details.insert("redis".into(), json!("ok")); true }
            Err(e) => {
                tracing::error!("Health check: Redis PING failed: {}", e);
                details.insert("redis".into(), json!(format!("error: {}", e)));
                false
            }
        },
        Err(e) => {
            tracing::error!("Health check: Redis connection failed: {}", e);
            details.insert("redis".into(), json!(format!("error: {}", e)));
            false
        }
    };
    if !redis_healthy { healthy = false; }

    let status = if healthy { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    let body = json!({ "status": if healthy { "ok" } else { "unhealthy" }, "details": details });
    (status, Json(body)).into_response()
}
