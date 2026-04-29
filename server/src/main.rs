use free_cake_server::{AppState, config, create_router, db, services::{audit_log::AuditLogService, notification::NotificationService}};
use sqlx::Row;
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = config::AppConfig::from_env();
    let db_pool = db::create_pool(&config.database_url).await;
    db::run_migrations(&db_pool).await;
    let redis_client = redis::Client::open(config.redis_url.clone())
        .expect("Failed to create Redis client");

    let sms_service = free_cake_server::services::sms::SmsService::from_env();
    let crypto_service = free_cake_server::services::crypto::CryptoService::from_env();
    let http_client = reqwest::Client::new();

    let state = AppState {
        db_pool: db_pool.clone(),
        redis_client,
        config: config.clone(),
        sms_service,
        crypto_service,
        http_client,
    };

    // Spawn background scheduler for activity transitions and order cleanup
    let scheduler_pool = db_pool.clone();
    let scheduler_handle = tokio::spawn(async move {
        run_scheduler(&scheduler_pool).await;
    });

    let app = create_router(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
        .await
        .unwrap();

    tracing::info!("Server running on port {}", config.server_port);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    // Cancel background scheduler on shutdown
    scheduler_handle.abort();
    tracing::info!("Scheduler cancelled, shutdown complete");
}

async fn run_scheduler(pool: &sqlx::PgPool) {
    let interval = std::time::Duration::from_secs(60);
    loop {
        tokio::time::sleep(interval).await;
        run_tick(pool).await;
    }
}

async fn run_tick(pool: &sqlx::PgPool) {
    // Each tick is fire-and-forget; a panic in one won't affect the others or the loop.
    // tokio::spawn isolates panics to the spawned task rather than aborting the scheduler.
    let p1 = pool.clone();
    tokio::spawn(async move {
        if let Err(e) = tick_activity_transitions(&p1).await {
            tracing::error!("Scheduler: activity transition failed: {}", e);
        }
    });
    let p2 = pool.clone();
    tokio::spawn(async move {
        if let Err(e) = tick_order_timeouts(&p2).await {
            tracing::error!("Scheduler: order timeout failed: {}", e);
        }
    });
    let p3 = pool.clone();
    tokio::spawn(async move {
        if let Err(e) = tick_auto_settle(&p3).await {
            tracing::error!("Scheduler: auto-settle failed: {}", e);
        }
    });
}

async fn tick_activity_transitions(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now();

    // Auto-open registration
    let result = sqlx::query(
        "UPDATE activity SET status = 'registration_open' WHERE status = 'draft' AND registration_start_at <= $1"
    )
    .bind(now)
    .execute(pool)
    .await?;
    if result.rows_affected() > 0 {
        tracing::info!("Scheduler: opened {} activities for registration", result.rows_affected());
    }

    // Auto-open voting
    let result = sqlx::query(
        "UPDATE activity SET status = 'voting_open' WHERE status = 'registration_open' AND voting_start_at <= $1"
    )
    .bind(now)
    .execute(pool)
    .await?;
    if result.rows_affected() > 0 {
        tracing::info!("Scheduler: opened {} activities for voting", result.rows_affected());
    }

    // Auto-close voting
    let result = sqlx::query(
        "UPDATE activity SET status = 'voting_closed' WHERE status = 'voting_open' AND voting_end_at <= $1"
    )
    .bind(now)
    .execute(pool)
    .await?;
    if result.rows_affected() > 0 {
        tracing::info!("Scheduler: closed voting for {} activities", result.rows_affected());
    }

    Ok(())
}

async fn tick_order_timeouts(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let cutoff = chrono::Utc::now() - chrono::Duration::minutes(30);
    let result = sqlx::query(
        "UPDATE reward_order SET pay_status = 'closed', closed_at = NOW() WHERE pay_status = 'pending' AND created_at < $1"
    )
    .bind(cutoff)
    .execute(pool)
    .await?;
    if result.rows_affected() > 0 {
        tracing::info!("Scheduler: closed {} timed-out orders", result.rows_affected());
    }
    Ok(())
}

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    tracing::info!("Shutting down gracefully...");
}

async fn tick_auto_settle(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let activities = sqlx::query(
        "SELECT id, max_winner_count, region_id FROM activity WHERE status = 'voting_closed' AND voting_end_at < NOW() - INTERVAL '5 minutes'"
    )
    .fetch_all(pool)
    .await?;

    for activity in &activities {
        let activity_id: i64 = activity.get("id");
        let max_winners: i32 = activity.get("max_winner_count");
        let region_id: i64 = activity.get("region_id");

        let mut tx = pool.begin().await?;

        // Re-check status inside transaction with row lock to avoid races
        let status_row = sqlx::query_scalar::<_, String>(
            "SELECT status FROM activity WHERE id = $1 FOR UPDATE"
        )
        .bind(activity_id)
        .fetch_one(&mut *tx)
        .await?;

        if status_row != "voting_closed" {
            tx.rollback().await?;
            continue;
        }

        let entries = sqlx::query(
            "SELECT id, user_id, valid_vote_count FROM contest_entry WHERE activity_id = $1 AND status = 'active' ORDER BY valid_vote_count DESC LIMIT $2"
        )
        .bind(activity_id)
        .bind(max_winners)
        .fetch_all(&mut *tx)
        .await?;

        let winner_count = entries.len() as i32;
        for (idx, entry) in entries.iter().enumerate() {
            let entry_id: i64 = entry.get("id");
            let user_id: i64 = entry.get("user_id");
            let valid_votes: i32 = entry.get("valid_vote_count");
            let rank = (idx + 1) as i32;

            let winner_row = sqlx::query(
                "INSERT INTO winner_record (activity_id, entry_id, user_id, rank, valid_vote_count, status) VALUES ($1, $2, $3, $4, $5, 'confirmed') RETURNING id"
            )
            .bind(activity_id)
            .bind(entry_id)
            .bind(user_id)
            .bind(rank)
            .bind(valid_votes)
            .fetch_one(&mut *tx)
            .await?;
            let winner_id: i64 = winner_row.get::<i64, _>("id");

            let stores = sqlx::query("SELECT id FROM store WHERE region_id = $1 AND status = 'active' LIMIT 1")
                .bind(region_id)
                .fetch_optional(&mut *tx)
                .await?;
            let store_id: i64 = stores.map(|s| s.get::<i64, _>("id")).unwrap_or(0);

            if store_id == 0 {
                tracing::warn!("Auto-settle: no active store for region_id={}", region_id);
            }

            let templates = sqlx::query("SELECT selected_template_id FROM contest_entry WHERE id = $1")
                .bind(entry_id)
                .fetch_one(&mut *tx)
                .await?;
            let template_id: i64 = templates.get("selected_template_id");

            let order_row = sqlx::query(
                "INSERT INTO reward_order (winner_id, store_id, order_type, template_id, production_status, redeem_status) VALUES ($1, $2, 'free', $3, 'pending', 'pending') RETURNING id"
            )
            .bind(winner_id)
            .bind(store_id)
            .bind(template_id)
            .fetch_one(&mut *tx)
            .await?;
            let order_id: i64 = order_row.get::<i64, _>("id");

            let code = uuid::Uuid::new_v4().to_string()[..8].to_string();
            sqlx::query(
                "INSERT INTO redeem_code (order_id, code, expires_at, status) VALUES ($1, $2, (SELECT voting_end_at FROM activity WHERE id = $3) + INTERVAL '7 days', 'valid')"
            )
            .bind(order_id)
            .bind(&code)
            .bind(activity_id)
            .execute(&mut *tx)
            .await?;
        }

        sqlx::query("UPDATE activity SET status = 'settled' WHERE id = $1")
            .bind(activity_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        AuditLogService::log_with_pool(pool, 0, "auto_settle", "activity", activity_id, &format!("winner_count={}", winner_count)).await;
        NotificationService::send_settle_notification(pool, activity_id).await;

        tracing::info!("Scheduler: auto-settled activity {} with {} winners", activity_id, winner_count);
    }

    Ok(())
}
