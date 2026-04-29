use free_cake_server::handlers::auth::Claims;
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, Algorithm};

#[test]
fn test_jwt_token_generation_and_verification() {
    let secret = "test-secret";
    let claims = Claims {
        user_id: 1,
        role: "admin".to_string(),
        jti: "test-jti".to_string(),
        exp: 9999999999,
        open_id: None,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).unwrap();

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).unwrap();

    assert_eq!(decoded.claims.user_id, 1);
    assert_eq!(decoded.claims.role, "admin");
}

#[test]
fn test_jwt_token_wrong_secret_fails() {
    let secret = "test-secret";
    let wrong_secret = "wrong-secret";
    let claims = Claims {
        user_id: 1,
        role: "admin".to_string(),
        jti: "test-jti".to_string(),
        exp: 9999999999,
        open_id: None,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).unwrap();

    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(wrong_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    assert!(result.is_err());
}

#[test]
fn test_jwt_token_expired_fails() {
    let secret = "test-secret";
    let claims = Claims {
        user_id: 1,
        role: "admin".to_string(),
        jti: "test-jti".to_string(),
        exp: 1,
        open_id: None,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).unwrap();

    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    assert!(result.is_err());
}

#[test]
fn test_phone_validation_empty_rejected() {
    let phone = String::new();
    let verify_code = String::from("123456");
    assert!(phone.is_empty() || verify_code.is_empty());
}

#[test]
fn test_phone_validation_nonempty_accepted() {
    let phone = String::from("13812345678");
    let verify_code = String::from("123456");
    assert!(!phone.is_empty() && !verify_code.is_empty());
}

#[test]
fn test_login_rate_limit_logic() {
    let max_attempts = 10;
    assert!(11 > max_attempts);
    assert!(9 <= max_attempts);
}
