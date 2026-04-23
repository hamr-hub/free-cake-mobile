use axum::response::Response;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use crate::errors::AppError;

#[allow(dead_code)]
pub async fn error_handler_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    Ok(next.run(req).await)
}
