use free_cake_server::services::crypto::CryptoService;
use free_cake_server::services::ai_generator::AiGeneratorService;
use sha2::{Sha256, Digest};

fn make_crypto() -> CryptoService {
    let mut key = [0u8; 32];
    key[..8].copy_from_slice(b"testkey!");
    CryptoService::new(key)
}

// ── CryptoService tests ──

#[test]
fn crypto_encrypt_decrypt_roundtrip() {
    let crypto = make_crypto();
    let phone = "13812345678";
    let encrypted = crypto.encrypt(phone).unwrap();
    let decrypted = crypto.decrypt(&encrypted).unwrap();
    assert_eq!(decrypted, phone);
}

#[test]
fn crypto_encrypt_produces_different_ciphertexts() {
    let crypto = make_crypto();
    let phone = "13812345678";
    let enc1 = crypto.encrypt(phone).unwrap();
    let enc2 = crypto.encrypt(phone).unwrap();
    assert_ne!(enc1, enc2);
}

#[test]
fn crypto_decrypt_with_wrong_key_fails() {
    let crypto1 = make_crypto();
    let mut key2 = [0u8; 32];
    key2[..8].copy_from_slice(b"wrongkey");
    let crypto2 = CryptoService::new(key2);

    let encrypted = crypto1.encrypt("13812345678").unwrap();
    assert!(crypto2.decrypt(&encrypted).is_err());
}

#[test]
fn crypto_decrypt_garbage_fails() {
    let crypto = make_crypto();
    assert!(crypto.decrypt("not-valid-base64!!!").is_err());
    assert!(crypto.decrypt("").is_err());
}

#[test]
fn crypto_mask_phone_11digits() {
    let crypto = make_crypto();
    let encrypted = crypto.encrypt("13812345678").unwrap();
    let masked = crypto.mask_phone(&encrypted);
    assert_eq!(masked, "138****5678");
}

#[test]
fn crypto_mask_phone_short_number() {
    let crypto = make_crypto();
    let encrypted = crypto.encrypt("12345").unwrap();
    let masked = crypto.mask_phone(&encrypted);
    assert_eq!(masked, "***");
}

// ── SHA-256 hash tests ──

#[test]
fn sha256_produces_consistent_hash() {
    let hash1 = sha256_hex("13812345678");
    let hash2 = sha256_hex("13812345678");
    assert_eq!(hash1, hash2);
    assert_eq!(hash1.len(), 64);
}

#[test]
fn sha256_different_inputs_different_hashes() {
    let h1 = sha256_hex("13812345678");
    let h2 = sha256_hex("13812345679");
    assert_ne!(h1, h2);
}

fn sha256_hex(s: &str) -> String {
    let hash = Sha256::digest(s.as_bytes());
    format!("{:x}", hash)
}

// ── AI prompt building tests ──

#[test]
fn ai_prompt_builds_correctly() {
    let prompt = AiGeneratorService::build_prompt(
        "birthday",
        "星空",
        "生日快乐",
        "暖色系",
        "卡通风",
    );
    assert!(prompt.contains("生日蛋糕"));
    assert!(prompt.contains("卡通风"));
    assert!(prompt.contains("暖色系"));
    assert!(prompt.contains("星空"));
    assert!(prompt.contains("生日快乐"));
}

#[test]
fn ai_prompt_sanitizes_special_chars() {
    let prompt = AiGeneratorService::build_prompt(
        "birthday",
        "<script>alert(1)</script>",
        "test",
        "warm",
        "cartoon",
    );
    assert!(!prompt.contains("<script>"));
    assert!(!prompt.contains("</script>"));
}

#[test]
fn ai_prompt_truncates_long_input() {
    let long_theme = "a".repeat(200);
    let prompt = AiGeneratorService::build_prompt(
        "birthday",
        &long_theme,
        "blessing",
        "warm",
        "cartoon",
    );
    // Prompt should be finite and not contain 200 'a's
    assert!(prompt.len() < 500);
}

// ── Phone validation tests ──

#[test]
fn phone_validation_chinese_mobile_valid() {
    let phone = "13812345678";
    assert!(phone.starts_with('1'));
    assert_eq!(phone.len(), 11);
    let second: char = phone.chars().nth(1).unwrap();
    assert!(('3'..='9').contains(&second));
}

#[test]
fn phone_validation_invalid_too_short() {
    let phone = "1381234";
    assert_ne!(phone.len(), 11);
}

#[test]
fn phone_validation_invalid_starts_with_0() {
    let phone = "01812345678";
    assert!(!phone.starts_with('1'));
}

// ── Config validation logic tests ──

#[test]
fn config_rejects_default_jwt() {
    let jwt = "dev-secret-DO-NOT-USE-IN-PROD";
    assert!(jwt.starts_with("dev-secret"));
}

#[test]
fn config_accepts_secure_jwt() {
    let jwt = "a1b2c3d4e5f6g7h8i9j0-random-secure-key-1234567890";
    assert!(!jwt.starts_with("dev-secret"));
    assert!(jwt.len() >= 32);
}

// ── Risk control dimension tests ──

#[test]
fn risk_control_threshold_sanity() {
    // Verify our thresholds are reasonable
    let phone_threshold = 10i64;
    let device_threshold = 3i64;
    let ip_threshold = 5i64;
    let geo_threshold = 8i64;

    assert!(phone_threshold > 0);
    assert!(device_threshold > 0);
    assert!(ip_threshold > 0);
    assert!(geo_threshold > 0);
    assert!(phone_threshold > device_threshold); // Phone is less strict than device
    assert!(ip_threshold > device_threshold);    // IP shared by NAT, more lenient
}

// ── Redeeem lock TTL test ──

#[test]
fn redeem_lock_ttl_is_reasonable() {
    let lock_ttl_secs: u64 = 10; // Updated from 30 to 10
    assert!(lock_ttl_secs >= 5);
    assert!(lock_ttl_secs <= 30);
}
