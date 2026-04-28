use axum::{extract::State, Json};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
pub use crate::app_middleware::auth::Claims;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub verify_code: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i64,
    pub role: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    if req.phone.is_empty() || req.verify_code.is_empty() {
        return Err(AppError::BadRequest("Phone and verify_code are required".into()));
    }

    let verify_key = format!("verify_code:{}:{}", req.phone, req.verify_code);
    let mut conn = state.redis_client.get_multiplexed_async_connection().await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let stored_code: Option<String> = redis::cmd("GET")
        .arg(&verify_key)
        .query_async::<Option<String>>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if stored_code.is_none() {
        return Err(AppError::Unauthorized("Invalid verify code".into()));
    }

    redis::cmd("DEL")
        .arg(&verify_key)
        .query_async::<()>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let rate_key = format!("login_rate:{}", req.phone);
    let count: i64 = redis::cmd("INCR")
        .arg(&rate_key)
        .query_async::<i64>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if count == 1 {
        redis::cmd("EXPIRE")
            .arg(&rate_key)
            .arg(3600)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    }
    if count > 10 {
        return Err(AppError::RateLimited("Too many login attempts".into()));
    }

    let user = sqlx::query("SELECT id, role FROM user WHERE phone = ?")
        .bind(&req.phone)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let (user_id, role) = match user {
        Some(row) => (row.get::<i64, _>("id"), row.get::<String, _>("role")),
        None => {
            let phone_hash = format!("{:x}", md5_hash(&req.phone));
            let result = sqlx::query(
                "INSERT INTO user (phone, phone_hash, role) VALUES (?, ?, 'user')"
            )
            .bind(&req.phone)
            .bind(&phone_hash)
            .execute(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            (result.last_insert_id() as i64, "user".to_string())
        }
    };

    let exp = Utc::now().timestamp() as u64 + state.config.jwt_expiration_hours * 3600;
    let claims = Claims { user_id, role: role.clone(), exp };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    ).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(LoginResponse { token, user_id, role }))
}

fn md5_hash(s: &str) -> u128 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish() as u128
}
