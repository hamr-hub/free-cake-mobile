use crate::errors::AppError;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

#[derive(Clone)]
pub struct SmsService {
    provider: SmsProvider,
}

enum SmsProvider {
    Dev,
    AlibabaCloud {
        access_key_id: String,
        access_key_secret: String,
        sign_name: String,
        template_code: String,
    },
    TencentCloud {
        secret_id: String,
        secret_key: String,
        sms_sdk_app_id: String,
        sign_name: String,
        template_id: String,
    },
}

impl Clone for SmsProvider {
    fn clone(&self) -> Self {
        match self {
            SmsProvider::Dev => SmsProvider::Dev,
            SmsProvider::AlibabaCloud { access_key_id, access_key_secret, sign_name, template_code } => SmsProvider::AlibabaCloud {
                access_key_id: access_key_id.clone(),
                access_key_secret: access_key_secret.clone(),
                sign_name: sign_name.clone(),
                template_code: template_code.clone(),
            },
            SmsProvider::TencentCloud { secret_id, secret_key, sms_sdk_app_id, sign_name, template_id } => SmsProvider::TencentCloud {
                secret_id: secret_id.clone(),
                secret_key: secret_key.clone(),
                sms_sdk_app_id: sms_sdk_app_id.clone(),
                sign_name: sign_name.clone(),
                template_id: template_id.clone(),
            },
        }
    }
}

impl SmsService {
    pub fn from_env() -> Self {
        let provider = match dotenvy::var("SMS_PROVIDER").unwrap_or_default().to_lowercase().as_str() {
            "alibaba" | "aliyun" => SmsProvider::AlibabaCloud {
                access_key_id: dotenvy::var("SMS_ACCESS_KEY_ID").unwrap_or_default(),
                access_key_secret: dotenvy::var("SMS_ACCESS_KEY_SECRET").unwrap_or_default(),
                sign_name: dotenvy::var("SMS_SIGN_NAME").unwrap_or_else(|_| "FreeCake".into()),
                template_code: dotenvy::var("SMS_TEMPLATE_CODE").unwrap_or_default(),
            },
            "tencent" | "tencentyun" => SmsProvider::TencentCloud {
                secret_id: dotenvy::var("SMS_SECRET_ID").unwrap_or_default(),
                secret_key: dotenvy::var("SMS_SECRET_KEY").unwrap_or_default(),
                sms_sdk_app_id: dotenvy::var("SMS_SDK_APP_ID").unwrap_or_default(),
                sign_name: dotenvy::var("SMS_SIGN_NAME").unwrap_or_else(|_| "FreeCake".into()),
                template_id: dotenvy::var("SMS_TEMPLATE_ID").unwrap_or_default(),
            },
            _ => {
                tracing::warn!("SMS_PROVIDER not set or unknown — using dev mode (log only)");
                SmsProvider::Dev
            }
        };
        Self { provider }
    }

    pub async fn send_verify_code(&self, phone: &str, code: &str) -> Result<(), AppError> {
        match &self.provider {
            SmsProvider::Dev => {
                tracing::info!("[DEV-SMS] verify code for {}: {}", phone, code);
                Ok(())
            }
            SmsProvider::AlibabaCloud { access_key_id, access_key_secret, sign_name, template_code } => {
                self.send_alibaba_cloud(phone, code, access_key_id, access_key_secret, sign_name, template_code).await
            }
            SmsProvider::TencentCloud { secret_id, secret_key, sms_sdk_app_id, sign_name, template_id } => {
                self.send_tencent_cloud(phone, code, secret_id, secret_key, sms_sdk_app_id, sign_name, template_id).await
            }
        }
    }

    async fn send_alibaba_cloud(
        &self,
        phone: &str,
        code: &str,
        access_key_id: &str,
        access_key_secret: &str,
        sign_name: &str,
        template_code: &str,
    ) -> Result<(), AppError> {
        let client = reqwest::Client::new();
        let template_param = serde_json::json!({ "code": code }).to_string();

        let mut params = vec![
            ("AccessKeyId", access_key_id.to_string()),
            ("Action", "SendSms".to_string()),
            ("Format", "JSON".to_string()),
            ("PhoneNumbers", phone.to_string()),
            ("RegionId", "cn-hangzhou".to_string()),
            ("SignName", sign_name.to_string()),
            ("SignatureMethod", "HMAC-SHA1".to_string()),
            ("SignatureNonce", uuid::Uuid::new_v4().to_string()),
            ("SignatureVersion", "1.0".to_string()),
            ("TemplateCode", template_code.to_string()),
            ("TemplateParam", template_param),
            ("Timestamp", chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            ("Version", "2017-05-25".to_string()),
        ];
        params.sort_by(|a, b| a.0.cmp(b.0));

        let canonicalized: String = params.iter()
            .map(|(k, v)| format!("{}={}", pct_encode(k), pct_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let string_to_sign = format!("GET&{}&{}", pct_encode("/"), pct_encode(&canonicalized));
        let signature = hmac_sha1_base64(&format!("{}&", access_key_secret), &string_to_sign);

        let url = format!(
            "https://dysmsapi.aliyuncs.com/?Signature={}&{}",
            pct_encode(&signature),
            canonicalized
        );

        let resp = client.get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Alibaba SMS request failed: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            tracing::error!("Alibaba SMS API error: status={}, body={}", status, body);
            return Err(AppError::Internal("SMS service error".into()));
        }

        let body: serde_json::Value = resp.json().await
            .map_err(|e| AppError::Internal(format!("Alibaba SMS response parse failed: {}", e)))?;

        if body["Code"].as_str().unwrap_or("") != "OK" {
            tracing::error!("Alibaba SMS send failed: {:?}", body);
            return Err(AppError::Internal(format!("SMS send failed: {}", body["Message"].as_str().unwrap_or("unknown"))));
        }

        tracing::info!("SMS sent successfully to {}", phone);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn send_tencent_cloud(
        &self,
        phone: &str,
        code: &str,
        secret_id: &str,
        secret_key: &str,
        sdk_app_id: &str,
        sign_name: &str,
        template_id: &str,
    ) -> Result<(), AppError> {
        let client = reqwest::Client::new();
        let host = "sms.tencentcloudapi.com";
        let action = "SendSms";
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let date = chrono::Utc::now().format("%Y-%m-%d").to_string();

        let body = serde_json::json!({
            "SmsSdkAppId": sdk_app_id,
            "SignName": sign_name,
            "TemplateId": template_id,
            "TemplateParamSet": [code],
            "PhoneNumberSet": [&format!("+86{}", phone)],
        });

        let payload = body.to_string();
        let hashed_payload = sha256_hex(&payload);
        let canonical_headers = format!("content-type:application/json; charset=utf-8\nhost:{}\nx-tc-action:{}\n", host, action.to_lowercase());
        let canonical_request = format!("POST\n/\n\n{}\n{}", canonical_headers, hashed_payload);
        let string_to_sign = format!("TC3-HMAC-SHA256\n{}\n{}/sms/tc3_request\n{}",
            timestamp, date, sha256_hex(&canonical_request));

        let secret_date = hmac_sha256_raw(format!("TC3{}", secret_key).as_bytes(), &date);
        let signing_key = hmac_sha256_raw(&secret_date, "sms");
        let signature = hmac_sha256_hex(&signing_key, &string_to_sign);

        let authorization = format!("TC3-HMAC-SHA256 Credential={}/{}/sms/tc3_request, SignedHeaders=content-type;host;x-tc-action, Signature={}",
            secret_id, date, signature);

        let resp = client.post(format!("https://{}", host))
            .header("Content-Type", "application/json; charset=utf-8")
            .header("Host", host)
            .header("X-TC-Action", action)
            .header("X-TC-Timestamp", &timestamp)
            .header("X-TC-Version", "2021-01-11")
            .header("X-TC-Region", "ap-guangzhou")
            .header("Authorization", &authorization)
            .body(payload)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Tencent SMS request failed: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_default();
            tracing::error!("Tencent SMS API error: status={}, body={}", status, body_text);
            return Err(AppError::Internal("SMS service error".into()));
        }

        let resp_body: serde_json::Value = resp.json().await
            .map_err(|e| AppError::Internal(format!("Tencent SMS response parse failed: {}", e)))?;

        let status_code = resp_body["Response"]["SendStatusSet"][0]["Code"].as_str().unwrap_or("");
        if status_code != "Ok" {
            tracing::error!("Tencent SMS send failed: {:?}", resp_body);
            return Err(AppError::Internal("SMS send failed".into()));
        }

        tracing::info!("SMS sent successfully to {}", phone);
        Ok(())
    }
}

fn pct_encode(s: &str) -> String {
    let mut result = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}

fn hmac_sha1_base64(key: &str, data: &str) -> String {
    let mut mac = HmacSha1::new_from_slice(key.as_bytes()).expect("HMAC key error");
    mac.update(data.as_bytes());
    let result = mac.finalize().into_bytes();
    base64::engine::general_purpose::STANDARD.encode(result)
}

fn sha256_hex(data: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hmac_sha256_raw(key: &[u8], data: &str) -> Vec<u8> {
    use hmac::Mac;
    type HmacSha256 = Hmac<sha2::Sha256>;
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key length error");
    mac.update(data.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

fn hmac_sha256_hex(key: &[u8], data: &str) -> String {
    let result = hmac_sha256_raw(key, data);
    result.iter().map(|b| format!("{:02x}", b)).collect()
}
