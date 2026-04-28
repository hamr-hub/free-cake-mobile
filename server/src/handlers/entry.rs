use axum::{extract::{State, Path, Extension}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::app_middleware::auth::Claims;
use crate::services::audit_log::AuditLogService;
use crate::services::ai_generator::AiGeneratorService;

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub scene: String,
    pub theme: String,
    pub blessing: String,
    pub color_preference: String,
    pub style: String,
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pub generation_id: i64,
    pub images: Vec<String>,
    pub template_ids: Vec<i64>,
}

#[derive(Deserialize)]
pub struct SubmitEntryRequest {
    pub selected_generation_id: i64,
    pub selected_template_id: i64,
    pub title: String,
}

#[derive(Serialize)]
pub struct SubmitEntryResponse {
    pub entry_id: i64,
    pub share_code: String,
}

pub async fn generate(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(activity_id): Path<i64>,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    let activity = sqlx::query("SELECT status FROM activity WHERE id = $1")
        .bind(activity_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = activity.ok_or_else(|| AppError::NotFound("Activity not found".into()))?;
    let status: String = row.get("status");
    if status != "registration_open" {
        return Err(AppError::BadRequest("Activity is not open for registration".into()));
    }

    let rate_key = format!("ai_gen_rate:{}", activity_id);
    let mut conn = state.redis_client.get_multiplexed_async_connection().await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let count: i64 = redis::cmd("INCR")
        .arg(&rate_key)
        .query_async::<i64>(&mut conn)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if count == 1 {
        redis::cmd("EXPIRE")
            .arg(&rate_key)
            .arg(3600)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    }
    let max_rate = state.config.ai_generation_rate_limit as i64;
    if count > max_rate {
        return Err(AppError::RateLimited("AI generation rate limit exceeded".into()));
    }

    let (images, template_ids) = AiGeneratorService::generate_cake_images(
        &state.config.ai_api_url,
        &state.config.ai_api_key,
        &req.scene,
        &req.theme,
        &req.blessing,
        &req.color_preference,
        &req.style,
    ).await?;

    let prompt = format!("{} {} {} {} {}", req.scene, req.theme, req.blessing, req.color_preference, req.style);
    let image_urls_str = serde_json::to_string(&images).unwrap_or_default();
    let template_ids_str = serde_json::to_string(&template_ids).unwrap_or_default();

    let row = sqlx::query(
        "INSERT INTO ai_generation_record (user_id, activity_id, scene, theme, blessing, color_preference, style, prompt, image_urls, template_ids, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'completed') RETURNING id"
    )
    .bind(claims.user_id)
    .bind(activity_id)
    .bind(&req.scene)
    .bind(&req.theme)
    .bind(&req.blessing)
    .bind(&req.color_preference)
    .bind(&req.style)
    .bind(&prompt)
    .bind(&image_urls_str)
    .bind(&template_ids_str)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(GenerateResponse {
        generation_id: row.get::<i64, _>("id"),
        images,
        template_ids,
    }))
}

pub async fn submit(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(activity_id): Path<i64>,
    Json(req): Json<SubmitEntryRequest>,
) -> Result<Json<SubmitEntryResponse>, AppError> {
    let activity = sqlx::query("SELECT status, region_id FROM activity WHERE id = $1")
        .bind(activity_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = activity.ok_or_else(|| AppError::NotFound("Activity not found".into()))?;
    let status: String = row.get("status");
    if status != "registration_open" {
        return Err(AppError::BadRequest("Activity is not open for entry submission".into()));
    }

    let generation = sqlx::query("SELECT image_urls, template_ids FROM ai_generation_record WHERE id = $1 AND activity_id = $2")
        .bind(req.selected_generation_id)
        .bind(activity_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::BadRequest("Generation record not found".into()))?;

    let image_urls_str: String = generation.get("image_urls");
    let images: Vec<String> = serde_json::from_str(&image_urls_str).unwrap_or_default();
    let selected_image = images.first().cloned().unwrap_or_default();

    let share_code = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let row = sqlx::query(
        "INSERT INTO contest_entry (activity_id, user_id, selected_generation_id, selected_template_id, title, share_code, image_url, status) VALUES ($1, $2, $3, $4, $5, $6, $7, 'active') RETURNING id"
    )
    .bind(activity_id)
    .bind(claims.user_id)
    .bind(req.selected_generation_id)
    .bind(req.selected_template_id)
    .bind(&req.title)
    .bind(&share_code)
    .bind(&selected_image)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(SubmitEntryResponse {
        entry_id: row.get::<i64, _>("id"),
        share_code,
    }))
}

#[derive(Deserialize)]
pub struct UpdateEntryStatusRequest {
    pub status: String,
}

pub async fn update_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(entry_id): Path<i64>,
    Json(req): Json<UpdateEntryStatusRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.status != "approved" && req.status != "rejected" && req.status != "active" {
        return Err(AppError::BadRequest("Invalid status, must be approved/rejected/active".into()));
    }

    let exists = sqlx::query("SELECT id, status FROM contest_entry WHERE id = $1")
        .bind(entry_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if exists.is_none() {
        return Err(AppError::NotFound("Entry not found".into()));
    }

    sqlx::query("UPDATE contest_entry SET status = $1 WHERE id = $2")
        .bind(&req.status)
        .bind(entry_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "entry_status_update", "contest_entry", entry_id, &format!("new_status={}", req.status)).await;

    Ok(Json(serde_json::json!({ "id": entry_id, "status": req.status })))
}

#[derive(Deserialize)]
pub struct FreezeEntryRequest {
    pub freeze: bool,
}

pub async fn freeze(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(entry_id): Path<i64>,
    Json(req): Json<FreezeEntryRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let new_status = if req.freeze { "frozen" } else { "active" };

    let exists = sqlx::query("SELECT id FROM contest_entry WHERE id = $1")
        .bind(entry_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if exists.is_none() {
        return Err(AppError::NotFound("Entry not found".into()));
    }

    sqlx::query("UPDATE contest_entry SET status = $1 WHERE id = $2")
        .bind(new_status)
        .bind(entry_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, if req.freeze { "entry_frozen" } else { "entry_unfrozen" }, "contest_entry", entry_id, &format!("freeze={}", req.freeze)).await;

    Ok(Json(serde_json::json!({ "id": entry_id, "status": new_status })))
}

#[derive(Deserialize)]
pub struct DeductVotesRequest {
    pub count: i32,
    pub reason: String,
}

pub async fn deduct_votes(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(entry_id): Path<i64>,
    Json(req): Json<DeductVotesRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.count <= 0 {
        return Err(AppError::BadRequest("Count must be positive".into()));
    }

    let entry = sqlx::query("SELECT id, valid_vote_count FROM contest_entry WHERE id = $1")
        .bind(entry_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Entry not found".into()))?;

    let current_votes: i32 = entry.get("valid_vote_count");
    let new_votes = current_votes - req.count;
    if new_votes < 0 {
        return Err(AppError::BadRequest(format!("Cannot deduct {} votes from entry with {} valid votes", req.count, current_votes)));
    }

    sqlx::query("UPDATE contest_entry SET valid_vote_count = $1 WHERE id = $2")
        .bind(new_votes)
        .bind(entry_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "votes_deducted", "contest_entry", entry_id, &format!("deducted={}, reason={}, before={}, after={}", req.count, req.reason, current_votes, new_votes)).await;

    Ok(Json(serde_json::json!({ "id": entry_id, "valid_vote_count": new_votes, "deducted": req.count })))
}
