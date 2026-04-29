use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

const DEFAULT_MAX_BODY_SIZE: usize = 1024 * 1024; // 1MB

pub async fn limit_body_middleware(req: Request, next: Next) -> Response {
    if let Some(content_length) = req.headers().get(header::CONTENT_LENGTH) {
        let max = DEFAULT_MAX_BODY_SIZE;
        match content_length.to_str() {
            Ok(s) => {
                if let Ok(len) = s.parse::<usize>() {
                    if len > max {
                        return (StatusCode::PAYLOAD_TOO_LARGE, "Request body too large").into_response();
                    }
                }
            }
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid Content-Length").into_response();
            }
        }
    }
    next.run(req).await
}
