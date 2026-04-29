use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use crate::AppState;
use crate::app_middleware::auth::ClientIp;

pub async fn ip_rate_limit_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let ip = request
        .extensions()
        .get::<ClientIp>()
        .map(|c| c.0.clone())
        .unwrap_or_default();

    if ip.is_empty() || ip == "unknown" {
        return next.run(request).await;
    }

    let key = format!("ratelimit:ip:{}", ip);
    let max_requests: i64 = 100;
    let window_secs: i64 = 60;

    let allowed = match state.redis_client.get_multiplexed_async_connection().await {
        Ok(mut conn) => {
            let count: i64 = redis::cmd("INCR")
                .arg(&key)
                .query_async(&mut conn)
                .await
                .unwrap_or(1);
            if count == 1 {
                let _: Result<(), _> = redis::cmd("EXPIRE")
                    .arg(&key)
                    .arg(window_secs)
                    .query_async(&mut conn)
                    .await;
            }
            count <= max_requests
        }
        Err(_) => true,
    };

    if allowed {
        next.run(request).await
    } else {
        (StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded").into_response()
    }
}
