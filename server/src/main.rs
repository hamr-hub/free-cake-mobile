use axum::{Router, middleware, routing::{get, post}};
use sqlx::MySqlPool;
use tower_http::cors::{CorsLayer, Any};

mod config;
mod db;
mod errors;
mod handlers;
mod app_middleware;
mod services;

use handlers::{auth, activity, entry, vote, settlement, order, redeem, inventory, store, dashboard};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
    pub redis_client: redis::Client,
    pub config: config::AppConfig,
}

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let public_routes = Router::new()
        .route("/auth/login", post(auth::login));

    let api_routes = Router::new()
        .route("/dashboard/stats", get(dashboard::stats))
        .route("/activities", post(activity::create).get(activity::list))
        .route("/activities/:id", get(activity::show))
        .route("/activities/:id/status", post(activity::update_status))
        .route("/activities/:id/entries/generate", post(entry::generate))
        .route("/activities/:id/entries", post(entry::submit))
        .route("/activities/:id/rank", get(vote::rank))
        .route("/activities/:id/settle", post(settlement::settle))
        .route("/entries/:id/vote", post(vote::cast))
        .route("/orders/:id/schedule", post(order::schedule))
        .route("/redeem/verify", post(redeem::verify))
        .route("/stores/:id/inventory", get(inventory::get_by_store))
        .route("/stores", post(store::create).get(store::list))
        .route("/stores/:id", get(store::show))
        .layer(middleware::from_fn_with_state(state.clone(), app_middleware::auth::auth_middleware));

    Router::new()
        .nest("/api", public_routes.merge(api_routes))
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = config::AppConfig::from_env();
    let db_pool = db::create_pool(&config.database_url).await;
    let redis_client = redis::Client::open(config.redis_url.clone())
        .expect("Failed to create Redis client");

    let state = AppState {
        db_pool,
        redis_client,
        config: config.clone(),
    };

    let app = create_router(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
        .await
        .unwrap();

    tracing::info!("Server running on port {}", config.server_port);
    axum::serve(listener, app).await.unwrap();
}
