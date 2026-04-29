use axum::{extract::{State, Extension, Multipart}, Json};
use crate::AppState;
use crate::errors::AppError;
use crate::services::audit_log::AuditLogService;
use crate::services::storage::StorageService;

const MAX_FILE_SIZE: usize = 5 * 1024 * 1024;
const ALLOWED_TYPES: [&str; 3] = ["image/jpeg", "image/png", "image/webp"];

pub async fn upload(
    State(state): State<AppState>,
    Extension(claims): Extension<crate::app_middleware::auth::Claims>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut file_data: Option<(String, String, Vec<u8>)> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(format!("Multipart error: {e}")))? {
        let name = field.name().unwrap_or("").to_string();
        if name != "file" {
            continue;
        }

        let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        if !ALLOWED_TYPES.contains(&content_type.as_str()) {
            return Err(AppError::BadRequest(format!("Unsupported file type: {content_type}")));
        }

        let data = field.bytes().await.map_err(|e| AppError::BadRequest(format!("Failed to read file: {e}")))?;
        if data.len() > MAX_FILE_SIZE {
            return Err(AppError::BadRequest("File too large (max 5MB)".into()));
        }

        let ext = match content_type.as_str() {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/webp" => "webp",
            _ => "bin",
        };
        let file_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
        file_data = Some((file_name, content_type, data.to_vec()));
        break;
    }

    let (file_name, content_type, data) = file_data.ok_or_else(|| AppError::BadRequest("No file provided".into()))?;

    if state.config.supabase_url.is_empty() {
        return Ok(Json(serde_json::json!({
            "url": format!("placeholder://upload/{}", file_name)
        })));
    }

    let url = StorageService::upload(
        &state.config.supabase_url,
        &state.config.supabase_api_key,
        &state.config.supabase_bucket,
        &file_name,
        &content_type,
        &data,
    ).await?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "upload", "file", 0, &format!("file={file_name}")).await;

    Ok(Json(serde_json::json!({ "url": url })))
}
