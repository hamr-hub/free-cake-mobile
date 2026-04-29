use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
    body::Body,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::AppError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub user_id: i64,
    pub role: String,
    pub jti: String,
    pub exp: u64,
    pub open_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ClientIp(pub String);

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header: Option<&str> = req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let auth_header = auth_header
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid token format".into()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|e| AppError::Unauthorized(format!("Token invalid: {}", e)))?;

    // Check JWT blacklist
    let blacklist_key = format!("jwt_blacklist:{}", token_data.claims.jti);
    if let Ok(mut conn) = state.redis_client.get_multiplexed_async_connection().await {
        let is_blacklisted: bool = redis::cmd("EXISTS")
            .arg(&blacklist_key)
            .query_async(&mut conn)
            .await
            .unwrap_or(0) > 0;
        if is_blacklisted {
            return Err(AppError::Unauthorized("Token has been revoked".into()));
        }
    }

    // Extract client IP from X-Forwarded-For (first entry) or fallback
    let client_ip = req.headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    req.extensions_mut().insert(token_data.claims);
    req.extensions_mut().insert(ClientIp(client_ip));

    Ok(next.run(req).await)
}

pub async fn admin_only_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let claims = req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::Unauthorized("Missing auth claims".into()))?;

    if claims.role != "admin" && claims.role != "operator" {
        return Err(AppError::Forbidden("Admin access required".into()));
    }

    Ok(next.run(req).await)
}
