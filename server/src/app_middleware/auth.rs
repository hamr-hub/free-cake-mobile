use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
    body::Body,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use crate::AppState;
use crate::errors::AppError;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Claims {
    pub user_id: i64,
    pub role: String,
    pub exp: u64,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header: Option<&str> = req.headers()
        .get("Authorization")
        .map(|v| v.to_str().ok())
        .flatten();

    let auth_header = auth_header
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid token format".into()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|e| AppError::Unauthorized(format!("Token invalid: {}", e)))?;

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
