use axum::{
    body::Body,
    extract::Request,
    http::{header, Response},
    middleware::Next,
};
use uuid::Uuid;

pub async fn request_id_middleware(request: Request, next: Next) -> Response<Body> {
    let request_id = request
        .headers()
        .get(header::HeaderName::from_static("x-request-id"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let response = next.run(request).await;

    let mut response = response;
    if let Ok(val) = header::HeaderValue::from_str(&request_id) {
        response.headers_mut().insert(
            header::HeaderName::from_static("x-request-id"),
            val,
        );
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_format() {
        let id = Uuid::new_v4().to_string();
        assert!(id.len() == 36);
        assert!(id.contains('-'));
    }
}
