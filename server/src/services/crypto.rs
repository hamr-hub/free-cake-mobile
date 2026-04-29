use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use rand::Rng;

#[derive(Clone)]
pub struct CryptoService {
    key: [u8; 32],
}

impl CryptoService {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn from_env() -> Self {
        let key_b64 = dotenvy::var("ENCRYPTION_KEY").unwrap_or_default();
        let key = if key_b64.is_empty() {
            tracing::warn!("ENCRYPTION_KEY not set — phone numbers will NOT be encrypted. Generate one with: openssl rand -base64 32");
            [0u8; 32]
        } else {
            BASE64.decode(&key_b64)
                .map_err(|_| tracing::error!("ENCRYPTION_KEY is not valid base64"))
                .map(|bytes| {
                    let mut key = [0u8; 32];
                    let len = bytes.len().min(32);
                    key[..len].copy_from_slice(&bytes[..len]);
                    key
                })
                .unwrap_or([0u8; 32])
        };
        Self { key }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| format!("Cipher init failed: {}", e))?;
        let nonce_bytes = rand::thread_rng().gen::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;
        let mut combined = Vec::with_capacity(12 + ciphertext.len());
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);
        Ok(BASE64.encode(&combined))
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String, String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| format!("Cipher init failed: {}", e))?;
        let combined = BASE64.decode(encrypted)
            .map_err(|e| format!("Base64 decode failed: {}", e))?;
        if combined.len() < 13 {
            return Err("Ciphertext too short".into());
        }
        let nonce = Nonce::from_slice(&combined[..12]);
        let ciphertext = &combined[12..];
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        String::from_utf8(plaintext).map_err(|e| format!("UTF-8 decode failed: {}", e))
    }

    pub fn mask_phone(&self, encrypted: &str) -> String {
        match self.decrypt(encrypted) {
            Ok(phone) => {
                if phone.len() == 11 {
                    format!("{}****{}", &phone[..3], &phone[7..])
                } else {
                    "***".to_string()
                }
            }
            Err(_) => "******".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_service() -> CryptoService {
        let key: [u8; 32] = [42u8; 32];
        CryptoService::new(key)
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let svc = test_service();
        let original = "13800138000";
        let encrypted = svc.encrypt(original).unwrap();
        assert_ne!(encrypted, original);
        let decrypted = svc.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original);
    }

    #[test]
    fn different_nonces_produce_different_ciphertexts() {
        let svc = test_service();
        let encrypted1 = svc.encrypt("13800138000").unwrap();
        let encrypted2 = svc.encrypt("13800138000").unwrap();
        assert_ne!(encrypted1, encrypted2);
    }

    #[test]
    fn decrypt_invalid_base64() {
        let svc = test_service();
        assert!(svc.decrypt("not-valid-base64!!!").is_err());
    }

    #[test]
    fn decrypt_too_short() {
        let svc = test_service();
        let short = BASE64.encode([0u8; 5]);
        assert!(svc.decrypt(&short).is_err());
    }

    #[test]
    fn decrypt_with_wrong_key() {
        let svc1 = CryptoService::new([42u8; 32]);
        let svc2 = CryptoService::new([99u8; 32]);
        let encrypted = svc1.encrypt("13800138000").unwrap();
        assert!(svc2.decrypt(&encrypted).is_err());
    }

    #[test]
    fn mask_phone_valid() {
        let svc = test_service();
        let encrypted = svc.encrypt("13800138000").unwrap();
        let masked = svc.mask_phone(&encrypted);
        assert_eq!(masked, "138****8000");
    }

    #[test]
    fn mask_phone_short_number() {
        let svc = test_service();
        let encrypted = svc.encrypt("12345").unwrap();
        let masked = svc.mask_phone(&encrypted);
        assert_eq!(masked, "***");
    }

    #[test]
    fn mask_phone_invalid_ciphertext() {
        let svc = test_service();
        let masked = svc.mask_phone("garbage");
        assert_eq!(masked, "******");
    }

    #[test]
    fn encrypt_empty_string() {
        let svc = test_service();
        let encrypted = svc.encrypt("").unwrap();
        let decrypted = svc.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, "");
    }

    #[test]
    fn encrypt_unicode() {
        let svc = test_service();
        let original = "用户名🎉";
        let encrypted = svc.encrypt(original).unwrap();
        let decrypted = svc.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original);
    }
}
