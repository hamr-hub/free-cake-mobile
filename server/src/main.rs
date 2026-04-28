use free_cake_server::{AppState, config, create_router, db};

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
