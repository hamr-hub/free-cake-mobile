pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod app_middleware;
pub mod services;

use axum::{Router, middleware, routing::{get, post}};
use sqlx::PgPool;
use tower_http::cors::{CorsLayer, Any};
use axum::http::HeaderValue;

use handlers::{auth, activity, entry, vote, settlement, order, redeem, inventory, store, dashboard, region, production, staff_handler, query_handlers, user};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_client: redis::Client,
    pub config: config::AppConfig,
}

pub fn create_router(state: AppState) -> Router {
    let cors = if state.config.cors_origin.is_empty() {
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
        .route("/auth/login", post(auth::login))
        .route("/auth/send-verify-code", post(query_handlers::send_verify_code));

    let admin_routes = Router::new()
        .route("/activities", post(activity::create).get(activity::list))
        .route("/activities/:id", get(activity::show))
        .route("/activities/:id/status", post(activity::update_status))
        .route("/activities/:id/entries/generate", post(entry::generate))
        .route("/activities/:id/entries", post(entry::submit))
        .route("/activities/:id/settle", post(settlement::settle))
        .route("/entries/:id/status", post(entry::update_status))
        .route("/entries/:id/freeze", post(entry::freeze))
        .route("/entries/:id/deduct", post(entry::deduct_votes))
        .route("/entries", get(query_handlers::entry_list))
        .route("/votes/risk", get(query_handlers::vote_risk_list))
        .route("/settlement", get(query_handlers::winner_list))
        .route("/production", get(query_handlers::production_list))
        .route("/redeem", get(query_handlers::redeem_list))
        .route("/audit_log", get(query_handlers::list))
        .route("/risk_events", get(query_handlers::risk_event_list))
        .route("/orders/:id/schedule", post(order::schedule))
        .route("/orders/:id/resend-code", post(order::resend_code))
        .route("/redeem/verify", post(redeem::verify))
        .route("/stores/:id/inventory", get(inventory::get_by_store))
        .route("/stores", post(store::create).get(store::list))
        .route("/stores/:id", get(store::show))
        .route("/regions", post(region::create).get(region::list))
        .route("/regions/:id", get(region::show))
        .route("/regions/:id/status", post(region::update_status))
        .route("/production/tasks/:id/complete", post(production::complete_task))
        .route("/staff", post(staff_handler::create).get(staff_handler::list))
        .route("/staff/:id", get(staff_handler::show))
        .route("/staff/:id/status", post(staff_handler::update_status))
        .route("/staff/check-in", post(staff_handler::check_in))
        .route("/staff/check-out", post(staff_handler::check_out))
        .route("/attendance", get(staff_handler::list_attendance))
        .layer(middleware::from_fn(app_middleware::auth::admin_only_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::auth::auth_middleware));

    let user_routes = Router::new()
        .route("/dashboard/stats", get(dashboard::stats))
        .route("/activities/:id/rank", get(vote::rank))
        .route("/entries/:id/vote", post(vote::cast))
        .route("/users/me", get(user::me))
        .route("/redeem/:code", get(user::redeem_detail))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::auth::auth_middleware));

    Router::new()
        .nest("/api", public_routes.merge(admin_routes).merge(user_routes))
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}
