#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
    pub ai_api_url: String,
    pub ai_api_key: String,
    pub server_port: u16,
    pub max_votes_per_day: i32,
    pub ai_generation_rate_limit: i32,
    pub enable_auto_settle: bool,
    pub risk_control_enabled: bool,
    pub cors_origin: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: dotenvy::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/free_cake".into()),
            redis_url: dotenvy::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".into()),
            jwt_secret: dotenvy::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".into()),
            jwt_expiration_hours: dotenvy::var("JWT_EXPIRATION_HOURS").unwrap_or_else(|_| "24".into()).parse().unwrap_or(24),
            ai_api_url: dotenvy::var("AI_API_URL").unwrap_or_else(|_| "".into()),
            ai_api_key: dotenvy::var("AI_API_KEY").unwrap_or_else(|_| "".into()),
            server_port: dotenvy::var("SERVER_PORT").unwrap_or_else(|_| "3000".into()).parse().unwrap_or(3000),
            max_votes_per_day: dotenvy::var("MAX_VOTES_PER_DAY").unwrap_or_else(|_| "3".into()).parse().unwrap_or(3),
            ai_generation_rate_limit: dotenvy::var("AI_GENERATION_RATE_LIMIT").unwrap_or_else(|_| "5".into()).parse().unwrap_or(5),
            enable_auto_settle: dotenvy::var("ENABLE_AUTO_SETTLE").unwrap_or_else(|_| "true".into()).parse().unwrap_or(true),
            risk_control_enabled: dotenvy::var("RISK_CONTROL_ENABLED").unwrap_or_else(|_| "true".into()).parse().unwrap_or(true),
            cors_origin: dotenvy::var("CORS_ORIGIN").unwrap_or_else(|_| "".into()),
        }
    }
}
