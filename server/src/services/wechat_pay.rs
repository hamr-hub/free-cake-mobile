use base64::Engine;
use rsa::pkcs8::DecodePrivateKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use sha2::Digest;
use crate::errors::AppError;

#[derive(Debug)]
pub struct WechatPayHeaders {
    pub timestamp: String,
    pub nonce: String,
    pub signature: String,
    pub serial: Option<String>,
}

impl WechatPayHeaders {
    pub fn from_headers(headers: &axum::http::HeaderMap) -> Result<Self, AppError> {
        let timestamp = headers
            .get("Wechatpay-Timestamp")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let nonce = headers
            .get("Wechatpay-Nonce")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let signature = headers
            .get("Wechatpay-Signature")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let serial = headers
            .get("Wechatpay-Serial")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        if timestamp.is_empty() || nonce.is_empty() || signature.is_empty() {
            return Err(AppError::Unauthorized("Missing WeChat Pay signature headers".into()));
        }

        if let Ok(ts) = timestamp.parse::<i64>() {
            let now = chrono::Utc::now().timestamp();
            if (now - ts).abs() > 300 {
                return Err(AppError::Unauthorized("WeChat Pay callback timestamp expired".into()));
            }
        }

        Ok(Self { timestamp, nonce, signature, serial })
    }
}

pub fn build_verify_message(timestamp: &str, nonce: &str, body: &str) -> String {
    format!("{}\n{}\n{}\n", timestamp, nonce, body)
}

pub fn verify_signature(
    headers: &WechatPayHeaders,
    body: &str,
    platform_cert_pem: &str,
) -> Result<(), AppError> {
    let message = build_verify_message(&headers.timestamp, &headers.nonce, body);

    let sig_bytes = base64::engine::general_purpose::STANDARD
        .decode(&headers.signature)
        .map_err(|_| AppError::Unauthorized("Invalid signature encoding".into()))?;

    let pem_parsed = pem::parse(platform_cert_pem)
        .map_err(|e| AppError::Internal(format!("Failed to parse platform cert PEM: {}", e)))?;

    let (_, cert) = x509_parser::parse_x509_certificate(pem_parsed.contents())
        .map_err(|e| AppError::Internal(format!("Failed to parse X509 cert: {}", e)))?;

    let parsed_pk = cert.subject_pki.parsed()
        .map_err(|e| AppError::Internal(format!("Failed to parse public key: {}", e)))?;

    let rsa_key = match parsed_pk {
        x509_parser::public_key::PublicKey::RSA(k) => k,
        _ => return Err(AppError::Internal("Platform cert does not contain an RSA key".into())),
    };

    let n = num_bigint_dig::BigUint::from_bytes_be(rsa_key.modulus);
    let e = num_bigint_dig::BigUint::from_bytes_be(rsa_key.exponent);

    let pub_key = rsa::RsaPublicKey::new(n, e)
        .map_err(|e| AppError::Internal(format!("Invalid RSA public key: {}", e)))?;

    let hash = sha2::Sha256::digest(message.as_bytes());

    pub_key
        .verify(rsa::pkcs1v15::Pkcs1v15Sign::new::<sha2::Sha256>(), &hash, &sig_bytes)
        .map_err(|_| AppError::Unauthorized("WeChat Pay signature verification failed".into()))?;

    tracing::info!("WeChat Pay callback signature verified (serial={:?})", headers.serial);
    Ok(())
}

pub fn verify_if_configured(
    headers: &axum::http::HeaderMap,
    body: &str,
    platform_cert_pem: &str,
) -> Result<(), AppError> {
    if platform_cert_pem.is_empty() {
        tracing::warn!("WECHAT_PAY_PLATFORM_CERT not set — accepting callback without signature verification (dev mode only)");
        return Ok(());
    }

    let wechat_headers = WechatPayHeaders::from_headers(headers)?;
    verify_signature(&wechat_headers, body, platform_cert_pem)
}

#[derive(Debug, serde::Serialize)]
pub struct JsapiOrderRequest {
    pub appid: String,
    pub mchid: String,
    pub description: String,
    pub out_trade_no: String,
    pub notify_url: String,
    pub amount: OrderAmount,
    pub payer: PayerInfo,
}

#[derive(Debug, serde::Serialize)]
pub struct OrderAmount {
    pub total: i64,
    pub currency: String,
}

#[derive(Debug, serde::Serialize)]
pub struct PayerInfo {
    pub openid: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct JsapiOrderResponse {
    pub prepay_id: String,
}

#[derive(Debug, serde::Serialize)]
pub struct PrepayParams {
    pub appid: String,
    pub partnerid: String,
    pub prepayid: String,
    pub package: String,
    pub noncestr: String,
    pub timestamp: String,
    pub sign_type: String,
    pub pay_sign: String,
}

pub struct JsapiOrderResult {
    pub order_id: i64,
    pub prepay_id: String,
    pub prepay_params: Option<PrepayParams>,
}

pub async fn create_jsapi_order(
    http_client: &reqwest::Client,
    config: &crate::config::AppConfig,
    order_id: i64,
    amount_fen: i64,
    description: &str,
    openid: &str,
) -> Result<JsapiOrderResult, AppError> {
    let mch_id = &config.wechat_pay_mch_id;
    let app_id = &config.wechat_app_id;

    if mch_id.is_empty() || config.wechat_pay_private_key.is_empty() || app_id.is_empty() {
        tracing::warn!("WeChat Pay/App credentials not configured — returning stub prepay_id for dev mode");
        let stub_prepay_id = format!("stub_prepay_{}", &uuid::Uuid::new_v4().to_string()[..8]);
        return Ok(JsapiOrderResult {
            order_id,
            prepay_id: stub_prepay_id,
            prepay_params: None,
        });
    }

    let out_trade_no = format!("ord_{}", order_id);
    let notify_url = config.wechat_pay_notify_url.clone();
    let body = JsapiOrderRequest {
        appid: app_id.clone(),
        mchid: mch_id.clone(),
        description: description.to_string(),
        out_trade_no: out_trade_no.clone(),
        notify_url: if notify_url.is_empty() {
            format!("https://your-domain.com/api/orders/{}/pay-callback", order_id)
        } else {
            notify_url
        },
        amount: OrderAmount {
            total: amount_fen,
            currency: "CNY".into(),
        },
        payer: PayerInfo {
            openid: openid.to_string(),
        },
    };

    let url = "https://api.mch.weixin.qq.com/v3/pay/transactions/jsapi";
    let url_path = "/v3/pay/transactions/jsapi";
    let body_json = serde_json::to_string(&body)
        .map_err(|e| AppError::Internal(format!("Failed to serialize order body: {}", e)))?;
    let auth_timestamp = chrono::Utc::now().timestamp().to_string();
    let auth_nonce = uuid::Uuid::new_v4().to_string()[..16].to_string();
    let auth_header = build_v3_auth_header(
        mch_id,
        &config.wechat_pay_serial_no,
        &auth_nonce,
        &auth_timestamp,
        "POST",
        url_path,
        &body_json,
        &config.wechat_pay_private_key,
    )?;

    let result = http_client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", auth_header)
        .body(body_json)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("JSAPI order request failed: {}", e)))?;

    if !result.status().is_success() {
        let status = result.status();
        let resp_body = result.text().await.unwrap_or_default();
        tracing::error!("WeChat JSAPI order API error: status={}, body={}", status, resp_body);
        return Err(AppError::Internal(format!("WeChat JSAPI order API returned {}: {}", status, resp_body)));
    }

    let resp: JsapiOrderResponse = result
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse JSAPI order response: {}", e)))?;

    let timestamp = chrono::Utc::now().timestamp().to_string();
    let nonce_str = uuid::Uuid::new_v4().to_string()[..16].to_string();
    let sign_type = "RSA".to_string();

    let sign_message = format!("{}\n{}\n{}\n{}\n", app_id, timestamp, nonce_str, "Sign=WXPay");
    let pay_sign = compute_jsapi_sign(&sign_message, &config.wechat_pay_private_key)
        .unwrap_or_else(|e| {
            tracing::error!("Failed to compute JSAPI pay_sign: {}", e);
            String::new()
        });

    Ok(JsapiOrderResult {
        order_id,
        prepay_id: resp.prepay_id.clone(),
        prepay_params: Some(PrepayParams {
            appid: app_id.clone(),
            partnerid: mch_id.clone(),
            prepayid: resp.prepay_id,
            package: "Sign=WXPay".into(),
            noncestr: nonce_str,
            timestamp,
            sign_type,
            pay_sign,
        }),
    })
}

#[derive(Debug, serde::Serialize)]
pub struct RefundRequest {
    pub out_trade_no: String,
    pub out_refund_no: String,
    pub reason: Option<String>,
    pub amount: RefundAmount,
}

#[derive(Debug, serde::Serialize)]
pub struct RefundAmount {
    pub refund: i64,
    pub total: i64,
    pub currency: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct RefundResponse {
    pub refund_id: String,
    pub out_refund_no: String,
    pub status: String,
}

pub async fn submit_refund(
    http_client: &reqwest::Client,
    config: &crate::config::AppConfig,
    out_trade_no: &str,
    refund_amount_fen: i64,
    total_amount_fen: i64,
) -> Result<String, AppError> {
    let mch_id = &config.wechat_pay_mch_id;

    if mch_id.is_empty() || config.wechat_pay_private_key.is_empty() {
        tracing::warn!("WeChat Pay merchant credentials not configured — returning stub refund ID");
        return Ok(format!("stub_refund_{}", &uuid::Uuid::new_v4().to_string()[..8]));
    }

    let out_refund_no = format!("rfn_{}", &uuid::Uuid::new_v4().to_string()[..12]);
    let body = RefundRequest {
        out_trade_no: out_trade_no.to_string(),
        out_refund_no: out_refund_no.clone(),
        reason: Some("admin refund".into()),
        amount: RefundAmount {
            refund: refund_amount_fen,
            total: total_amount_fen,
            currency: "CNY".into(),
        },
    };

    let url = "https://api.mch.weixin.qq.com/v3/refund/domestic/refunds";
    let url_path = "/v3/refund/domestic/refunds";
    let body_json = serde_json::to_string(&body)
        .map_err(|e| AppError::Internal(format!("Failed to serialize refund body: {}", e)))?;
    let auth_timestamp = chrono::Utc::now().timestamp().to_string();
    let auth_nonce = uuid::Uuid::new_v4().to_string()[..16].to_string();
    let auth_header = build_v3_auth_header(
        mch_id,
        &config.wechat_pay_serial_no,
        &auth_nonce,
        &auth_timestamp,
        "POST",
        url_path,
        &body_json,
        &config.wechat_pay_private_key,
    )?;

    let result = http_client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", auth_header)
        .body(body_json)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Refund API request failed: {}", e)))?;

    if !result.status().is_success() {
        let status = result.status();
        let body = result.text().await.unwrap_or_default();
        return Err(AppError::Internal(format!(
            "WeChat Refund API returned {}: {}", status, body
        )));
    }

    let resp: RefundResponse = result
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse refund response: {}", e)))?;

    Ok(resp.refund_id)
}

pub fn compute_jsapi_sign(message: &str, private_key_pem: &str) -> Result<String, AppError> {
    if private_key_pem.is_empty() {
        return Err(AppError::Internal("WeChat Pay private key not configured".into()));
    }

    let pem_parsed = pem::parse(private_key_pem)
        .map_err(|e| AppError::Internal(format!("Failed to parse private key PEM: {}", e)))?;

    let priv_key = rsa::RsaPrivateKey::from_pkcs8_der(pem_parsed.contents())
        .map_err(|e| AppError::Internal(format!("Failed to load RSA private key: {}", e)))?;

    let signing_key = rsa::pkcs1v15::SigningKey::<sha2::Sha256>::new(priv_key);
    let signature = signing_key.sign_with_rng(&mut rand::rngs::OsRng, message.as_bytes());

    Ok(base64::engine::general_purpose::STANDARD.encode(&*signature.to_bytes()))
}

fn build_v3_auth_header(
    mch_id: &str,
    serial_no: &str,
    nonce_str: &str,
    timestamp: &str,
    method: &str,
    url_path: &str,
    body: &str,
    private_key_pem: &str,
) -> Result<String, AppError> {
    let auth_sign_message = format!("{}\n{}\n{}\n{}\n{}\n", method, url_path, timestamp, nonce_str, body);
    let signature = compute_jsapi_sign(&auth_sign_message, private_key_pem)?;

    Ok(format!(
        r#"WECHATPAY2-SHA256-RSA2048 mchid="{}",nonce_str="{}",timestamp="{}",serial_no="{}",signature="{}""#,
        mch_id, nonce_str, timestamp, serial_no, signature
    ))
}
#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn missing_headers_rejected() {
        let headers = axum::http::HeaderMap::new();
        assert!(WechatPayHeaders::from_headers(&headers).is_err());
    }

    #[test]
    fn partial_headers_rejected() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("Wechatpay-Timestamp", "1234567890".parse().unwrap());
        headers.insert("Wechatpay-Nonce", "abc123".parse().unwrap());
        assert!(WechatPayHeaders::from_headers(&headers).is_err());
    }

    #[test]
    fn valid_headers_parsed() {
        let mut headers = axum::http::HeaderMap::new();
        let ts = chrono::Utc::now().timestamp().to_string();
        headers.insert("Wechatpay-Timestamp", ts.parse().unwrap());
        headers.insert("Wechatpay-Nonce", "test-nonce".parse().unwrap());
        headers.insert("Wechatpay-Signature", "dGVzdHNpZw==".parse().unwrap());
        headers.insert("Wechatpay-Serial", "cert-001".parse().unwrap());

        let result = WechatPayHeaders::from_headers(&headers);
        assert!(result.is_ok());
        let h = result.unwrap();
        assert_eq!(h.nonce, "test-nonce");
        assert_eq!(h.serial, Some("cert-001".into()));
    }

    #[test]
    fn empty_cert_allows_dev_mode() {
        let headers = axum::http::HeaderMap::new();
        assert!(verify_if_configured(&headers, "{}", "").is_ok());
    }

    #[test]
    fn message_format() {
        let msg = build_verify_message("1700000000", "abc123", "body");
        assert_eq!(msg, "1700000000\nabc123\nbody\n");
    }

    #[test]
    fn jsapi_order_request_serialization() {
        let req = JsapiOrderRequest {
            appid: "wx1234567890".into(),
            mchid: "mch_12345".into(),
            description: "6寸动物奶油蛋糕".into(),
            out_trade_no: "ord_100".into(),
            notify_url: "https://example.com/callback".into(),
            amount: OrderAmount { total: 2990, currency: "CNY".into() },
            payer: PayerInfo { openid: "oXYZ123".into() },
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appid\":\"wx1234567890\""));
        assert!(json.contains("\"total\":2990"));
        assert!(json.contains("\"openid\":\"oXYZ123\""));
    }

    #[test]
    fn prepay_params_fields() {
        let params = PrepayParams {
            appid: "wx1234567890".into(),
            partnerid: "mch_12345".into(),
            prepayid: "prepay_id_abc".into(),
            package: "Sign=WXPay".into(),
            noncestr: "abc1234567890123".into(),
            timestamp: "1700000000".into(),
            sign_type: "RSA".into(),
            pay_sign: String::new(),
        };
        assert_eq!(params.appid, "wx1234567890");
        assert_eq!(params.package, "Sign=WXPay");
    }

    #[test]
    fn compute_jsapi_sign_empty_key_rejected() {
        let result = compute_jsapi_sign("test message", "");
        assert!(result.is_err());
    }

    #[test]
    fn compute_jsapi_sign_valid_key() {
        use rsa::pkcs8::EncodePrivateKey;
        let priv_key = rsa::RsaPrivateKey::new(&mut rand::rngs::OsRng, 2048).unwrap();
        let priv_key_pem = priv_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap();
        let result = compute_jsapi_sign("wx1234567890\n1700000000\nabc1234567890123\nSign=WXPay\n", &priv_key_pem);
        assert!(result.is_ok());
        let sig = result.unwrap();
        let decoded = base64::engine::general_purpose::STANDARD.decode(&sig);
        assert!(decoded.is_ok());
        assert!(!decoded.unwrap().is_empty());
    }

    #[test]
    fn build_v3_auth_header_empty_key_rejected() {
        let result = build_v3_auth_header(
            "mch_123", "serial_001", "nonce", "1700000000",
            "POST", "/v3/pay/transactions/jsapi", "{}", "",
        );
        assert!(result.is_err());
    }
}
