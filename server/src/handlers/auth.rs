use axum::{extract::{State, Extension}, Json};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::services::validation;
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
    validation::validate_phone(&req.phone)?;
    validation::validate_verify_code(&req.verify_code)?;

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

    let phone_hash = sha256_hex(&req.phone);
    let phone_encrypted = state.crypto_service.encrypt(&req.phone)
        .map_err(AppError::Internal)?;

    let user = sqlx::query("SELECT id, role FROM app_user WHERE phone_hash = $1")
        .bind(&phone_hash)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let (user_id, role) = match user {
        Some(row) => (row.get::<i64, _>("id"), row.get::<String, _>("role")),
        None => {
            let row = sqlx::query(
                "INSERT INTO app_user (phone, phone_hash, phone_encrypted, role) VALUES ($1, $2, $3, 'user') RETURNING id"
            )
            .bind(&req.phone)
            .bind(&phone_hash)
            .bind(&phone_encrypted)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            (row.get::<i64, _>("id"), "user".to_string())
        }
    };

    let exp = Utc::now().timestamp() as u64 + state.config.jwt_expiration_hours * 3600;
    let jti = uuid::Uuid::new_v4().to_string();
    let claims = Claims { user_id, role: role.clone(), jti, exp, open_id: None };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    ).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(LoginResponse { token, user_id, role }))
}

fn sha256_hex(s: &str) -> String {
    use sha2::{Sha256, Digest};
    let hash = Sha256::digest(s.as_bytes());
    format!("{:x}", hash)
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub token: String,
    pub user_id: i64,
    pub role: String,
}

pub async fn refresh(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<RefreshResponse>, AppError> {
    let exp = Utc::now().timestamp() as u64 + state.config.jwt_expiration_hours * 3600;
    let jti = uuid::Uuid::new_v4().to_string();
    let new_claims = Claims { user_id: claims.user_id, role: claims.role.clone(), jti, exp, open_id: claims.open_id.clone() };
    let token = encode(
        &Header::default(),
        &new_claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    ).map_err(|e| AppError::Internal(e.to_string()))?;

    // Blacklist old token's jti with remaining TTL
    let remaining_secs = (claims.exp as i64 - Utc::now().timestamp()).max(0) as u64;
    if remaining_secs > 0 {
        if let Ok(mut conn) = state.redis_client.get_multiplexed_async_connection().await {
            let blacklist_key = format!("jwt_blacklist:{}", claims.jti);
            let _: Result<(), _> = redis::cmd("SET")
                .arg(&blacklist_key)
                .arg("1")
                .arg("EX")
                .arg(remaining_secs)
                .query_async(&mut conn)
                .await;
        }
    }

    Ok(Json(RefreshResponse { token, user_id: claims.user_id, role: claims.role }))
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

pub async fn logout(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<LogoutResponse>, AppError> {
    let remaining_secs = (claims.exp as i64 - Utc::now().timestamp()).max(0) as u64;
    if remaining_secs > 0 {
        if let Ok(mut conn) = state.redis_client.get_multiplexed_async_connection().await {
            let blacklist_key = format!("jwt_blacklist:{}", claims.jti);
            let _: Result<(), _> = redis::cmd("SET")
                .arg(&blacklist_key)
                .arg("1")
                .arg("EX")
                .arg(remaining_secs)
                .query_async(&mut conn)
                .await;
        }
    }
    Ok(Json(LogoutResponse { message: "Logged out".into() }))
}

#[derive(Deserialize)]
pub struct WechatLoginRequest {
    pub code: String,
}

#[derive(Serialize)]
pub struct WechatLoginResponse {
    pub token: Option<String>,
    pub user_id: Option<i64>,
    pub role: Option<String>,
    pub need_bind_phone: bool,
    pub openid: String,
}

pub async fn wechat_login(
    State(state): State<AppState>,
    Json(req): Json<WechatLoginRequest>,
) -> Result<Json<WechatLoginResponse>, AppError> {
    if req.code.is_empty() {
        return Err(AppError::BadRequest("WeChat login code is required".into()));
    }

    let app_id = dotenvy::var("WECHAT_APP_ID").unwrap_or_default();
    let app_secret = dotenvy::var("WECHAT_APP_SECRET").unwrap_or_default();

    if app_id.is_empty() || app_secret.is_empty() {
        return Err(AppError::Internal("WECHAT_APP_ID / WECHAT_APP_SECRET not configured".into()));
    }

    let client = reqwest::Client::new();
    let resp = client.get("https://api.weixin.qq.com/sns/jscode2session")
        .query(&[
            ("appid", &app_id),
            ("secret", &app_secret),
            ("js_code", &req.code),
            ("grant_type", &"authorization_code".to_string()),
        ])
        .timeout(std::time::Duration::from_secs(10))
        .send().await
        .map_err(|e| AppError::Internal(format!("WeChat API call failed: {}", e)))?;

    let body: serde_json::Value = resp.json().await
        .map_err(|e| AppError::Internal(format!("WeChat API response parse failed: {}", e)))?;

    let openid = body["openid"].as_str().unwrap_or("").to_string();
    if openid.is_empty() {
        let errcode = body["errcode"].as_i64().unwrap_or(-1);
        return Err(AppError::Internal(format!("WeChat login failed: errcode={}", errcode)));
    }

    // Check if openid is already bound to a user
    let existing = sqlx::query(
        "SELECT u.id, u.role FROM app_user u JOIN user_identity i ON u.id = i.user_id WHERE i.identity_type = 'wechat' AND i.identity_value = $1"
    )
    .bind(&openid)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    match existing {
        Some(row) => {
            let user_id: i64 = row.get("id");
            let role: String = row.get("role");
            let exp = Utc::now().timestamp() as u64 + state.config.jwt_expiration_hours * 3600;
            let jti = uuid::Uuid::new_v4().to_string();
    let claims = Claims { user_id, role: role.clone(), jti, exp, open_id: Some(openid.clone()) };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
            ).map_err(|e| AppError::Internal(e.to_string()))?;

            Ok(Json(WechatLoginResponse {
                token: Some(token),
                user_id: Some(user_id),
                role: Some(role),
                need_bind_phone: false,
                openid,
            }))
        }
        None => {
            Ok(Json(WechatLoginResponse {
                token: None,
                user_id: None,
                role: None,
                need_bind_phone: true,
                openid,
            }))
        }
    }
}

#[derive(Deserialize)]
pub struct BindPhoneRequest {
    pub openid: String,
    pub phone: String,
    pub verify_code: String,
}

#[derive(Serialize)]
pub struct BindPhoneResponse {
    pub token: String,
    pub user_id: i64,
    pub role: String,
}

pub async fn bind_phone(
    State(state): State<AppState>,
    Json(req): Json<BindPhoneRequest>,
) -> Result<Json<BindPhoneResponse>, AppError> {
    if req.openid.is_empty() || req.phone.is_empty() || req.verify_code.is_empty() {
        return Err(AppError::BadRequest("openid, phone, and verify_code are required".into()));
    }

    if !req.phone.starts_with('1') || req.phone.len() != 11 {
        return Err(AppError::BadRequest("Invalid phone number format".into()));
    }

    // Verify the SMS code
    let verify_key = format!("verify_code:{}:{}", req.phone, req.verify_code);
    let mut conn = state.redis_client.get_multiplexed_async_connection().await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let stored: Option<String> = redis::cmd("GET")
        .arg(&verify_key)
        .query_async::<Option<String>>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if stored.is_none() {
        return Err(AppError::Unauthorized("Invalid verify code".into()));
    }
    redis::cmd("DEL").arg(&verify_key).query_async::<()>(&mut conn).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let phone_hash = sha256_hex(&req.phone);
    let phone_encrypted = state.crypto_service.encrypt(&req.phone)
        .map_err(AppError::Internal)?;

    // Find or create the user by phone
    let user = sqlx::query("SELECT id, role FROM app_user WHERE phone_hash = $1")
        .bind(&phone_hash)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let (user_id, role) = match user {
        Some(row) => (row.get::<i64, _>("id"), row.get::<String, _>("role")),
        None => {
            let row = sqlx::query(
                "INSERT INTO app_user (phone, phone_hash, phone_encrypted, role) VALUES ($1, $2, $3, 'user') RETURNING id"
            )
            .bind(&req.phone)
            .bind(&phone_hash)
            .bind(&phone_encrypted)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
            (row.get::<i64, _>("id"), "user".to_string())
        }
    };

    // Bind openid to user via user_identity, update open_id on app_user
    sqlx::query(
        "INSERT INTO user_identity (user_id, identity_type, identity_value) VALUES ($1, 'wechat', $2) ON CONFLICT DO NOTHING"
    )
    .bind(user_id)
    .bind(&req.openid)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    sqlx::query("UPDATE app_user SET open_id = $1 WHERE id = $2")
        .bind(&req.openid)
        .bind(user_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let exp = Utc::now().timestamp() as u64 + state.config.jwt_expiration_hours * 3600;
    let jti = uuid::Uuid::new_v4().to_string();
    let claims = Claims { user_id, role: role.clone(), jti, exp, open_id: None };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    ).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(BindPhoneResponse { token, user_id, role }))
}
