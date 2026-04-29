use axum::response::Response;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use crate::errors::AppError;

pub async fn error_handler_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let result = next.run(req).await;
    Ok(result)
}
