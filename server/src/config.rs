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
    pub risk_control_enabled: bool,
    pub cors_origin: String,
    pub supabase_url: String,
    pub supabase_api_key: String,
    pub supabase_bucket: String,
    pub wechat_pay_api_key: String,
    pub wechat_pay_mch_id: String,
    pub wechat_pay_platform_cert: String,
    pub wechat_pay_private_key: String,
    pub wechat_pay_serial_no: String,
    pub wechat_app_id: String,
    pub wechat_pay_notify_url: String,
    pub sms_provider: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let config = Self {
            database_url: dotenvy::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/free_cake".into()),
            redis_url: dotenvy::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".into()),
            jwt_secret: dotenvy::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-DO-NOT-USE-IN-PROD".into()),
            jwt_expiration_hours: dotenvy::var("JWT_EXPIRATION_HOURS").unwrap_or_else(|_| "24".into()).parse().unwrap_or(24),
            ai_api_url: dotenvy::var("AI_API_URL").unwrap_or_else(|_| "".into()),
            ai_api_key: dotenvy::var("AI_API_KEY").unwrap_or_else(|_| "".into()),
            server_port: dotenvy::var("SERVER_PORT").unwrap_or_else(|_| "3000".into()).parse().unwrap_or(3000),
            max_votes_per_day: dotenvy::var("MAX_VOTES_PER_DAY").unwrap_or_else(|_| "3".into()).parse().unwrap_or(3),
            ai_generation_rate_limit: dotenvy::var("AI_GENERATION_RATE_LIMIT").unwrap_or_else(|_| "5".into()).parse().unwrap_or(5),
            risk_control_enabled: dotenvy::var("RISK_CONTROL_ENABLED").unwrap_or_else(|_| "true".into()).parse().unwrap_or(true),
            cors_origin: dotenvy::var("CORS_ORIGIN").unwrap_or_else(|_| "".into()),
            supabase_url: dotenvy::var("SUPABASE_URL").unwrap_or_else(|_| "".into()),
            supabase_api_key: dotenvy::var("SUPABASE_API_KEY").unwrap_or_else(|_| "".into()),
            supabase_bucket: dotenvy::var("SUPABASE_BUCKET").unwrap_or_else(|_| "uploads".into()),
            wechat_pay_api_key: dotenvy::var("WECHAT_PAY_API_KEY").unwrap_or_else(|_| "".into()),
            wechat_pay_mch_id: dotenvy::var("WECHAT_PAY_MCH_ID").unwrap_or_else(|_| "".into()),
            wechat_pay_platform_cert: dotenvy::var("WECHAT_PAY_PLATFORM_CERT").unwrap_or_else(|_| "".into()),
            wechat_pay_private_key: dotenvy::var("WECHAT_PAY_PRIVATE_KEY").unwrap_or_else(|_| "".into()),
            wechat_pay_serial_no: dotenvy::var("WECHAT_PAY_SERIAL_NO").unwrap_or_else(|_| "".into()),
            wechat_app_id: dotenvy::var("WECHAT_APP_ID").unwrap_or_else(|_| "".into()),
            wechat_pay_notify_url: dotenvy::var("WECHAT_PAY_NOTIFY_URL").unwrap_or_else(|_| "".into()),
            sms_provider: dotenvy::var("SMS_PROVIDER").unwrap_or_else(|_| "dev".into()),
        };
        config.validate();
        config
    }

    fn validate(&self) {
        let is_prod = dotenvy::var("APP_ENV").unwrap_or_default() == "production";
        if self.jwt_secret.starts_with("dev-secret") {
            if is_prod {
                panic!("FATAL: JWT_SECRET must be set to a secure value in production. Current value is the insecure default.");
            } else {
                tracing::warn!("JWT_SECRET is using insecure default. Do NOT use in production!");
            }
        }
        if is_prod && self.cors_origin.is_empty() {
            panic!("FATAL: CORS_ORIGIN must be set in production (comma-separated origins). Empty = allow all origins.");
        }
        if is_prod && self.database_url.contains("password@localhost") {
            panic!("FATAL: DATABASE_URL appears to use default dev credentials. Set a secure connection string in production.");
        }
    }

    pub fn is_insecure_jwt(&self) -> bool {
        self.jwt_secret.starts_with("dev-secret")
    }

    pub fn is_dev_database(&self) -> bool {
        self.database_url.contains("password@localhost")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> AppConfig {
        AppConfig {
            database_url: "postgres://user:pass@localhost/db".into(),
            redis_url: "redis://localhost:6379".into(),
            jwt_secret: "secure-random-string".into(),
            jwt_expiration_hours: 24,
            ai_api_url: String::new(),
            ai_api_key: String::new(),
            server_port: 3000,
            max_votes_per_day: 3,
            ai_generation_rate_limit: 5,
            risk_control_enabled: true,
            cors_origin: "https://example.com".into(),
            supabase_url: String::new(),
            supabase_api_key: String::new(),
            supabase_bucket: "uploads".into(),
            wechat_pay_api_key: String::new(),
            wechat_pay_mch_id: String::new(),
            wechat_pay_platform_cert: String::new(),
            wechat_pay_private_key: String::new(),
            wechat_pay_serial_no: String::new(),
            wechat_app_id: String::new(),
            wechat_pay_notify_url: String::new(),
            sms_provider: "dev".into(),
        }
    }

    #[test]
    fn secure_jwt_detected() {
        let config = test_config();
        assert!(!config.is_insecure_jwt());
    }

    #[test]
    fn insecure_jwt_detected() {
        let mut config = test_config();
        config.jwt_secret = "dev-secret-DO-NOT-USE-IN-PROD".into();
        assert!(config.is_insecure_jwt());
    }

    #[test]
    fn dev_database_detected() {
        let mut config = test_config();
        config.database_url = "postgres://postgres:password@localhost:5432/free_cake".into();
        assert!(config.is_dev_database());
    }

    #[test]
    fn prod_database_not_flagged() {
        let config = test_config();
        assert!(!config.is_dev_database());
    }
}
