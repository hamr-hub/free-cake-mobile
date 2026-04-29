use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use crate::AppState;

pub async fn public_rate_limit_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Stricter limits for auth endpoints
    let path = request.uri().path();
    let (max_requests, window_secs) = if path.contains("/auth/send-verify-code") {
        (10, 60) // 10 per minute for SMS
    } else if path.contains("/auth/") {
        (20, 60) // 20 per minute for other auth
    } else {
        (100, 60) // 100 per minute for other public
    };

    let key = format!("ratelimit:public:{}:{}", ip, path);
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
        Err(_) => true, // Fail open
    };

    if allowed {
        next.run(request).await
    } else {
        (StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded").into_response()
    }
}
