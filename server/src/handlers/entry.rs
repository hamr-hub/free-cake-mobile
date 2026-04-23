use axum::{extract::{State, Path}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
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
    Path(activity_id): Path<i64>,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    let activity = sqlx::query("SELECT status FROM activity WHERE id = ?")
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

    let result = sqlx::query(
        "INSERT INTO ai_generation_record (user_id, activity_id, scene, theme, blessing, color_preference, style, prompt, image_urls, template_ids, status) VALUES (0, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'completed')"
    )
    .bind(activity_id)
    .bind(&req.scene)
    .bind(&req.theme)
    .bind(&req.blessing)
    .bind(&req.color_preference)
    .bind(&req.style)
    .bind(&prompt)
    .bind(&image_urls_str)
    .bind(&template_ids_str)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(GenerateResponse {
        generation_id: result.last_insert_id() as i64,
        images,
        template_ids,
    }))
}

pub async fn submit(
    State(state): State<AppState>,
    Path(activity_id): Path<i64>,
    Json(req): Json<SubmitEntryRequest>,
) -> Result<Json<SubmitEntryResponse>, AppError> {
    let activity = sqlx::query("SELECT status, region_id FROM activity WHERE id = ?")
        .bind(activity_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = activity.ok_or_else(|| AppError::NotFound("Activity not found".into()))?;
    let status: String = row.get("status");
    if status != "registration_open" {
        return Err(AppError::BadRequest("Activity is not open for entry submission".into()));
    }

    let generation = sqlx::query("SELECT image_urls, template_ids FROM ai_generation_record WHERE id = ? AND activity_id = ?")
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

    let result = sqlx::query(
        "INSERT INTO contest_entry (activity_id, user_id, selected_generation_id, selected_template_id, title, share_code, image_url, status) VALUES (?, 0, ?, ?, ?, ?, ?, 'active')"
    )
    .bind(activity_id)
    .bind(req.selected_generation_id)
    .bind(req.selected_template_id)
    .bind(&req.title)
    .bind(&share_code)
    .bind(&selected_image)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(SubmitEntryResponse {
        entry_id: result.last_insert_id() as i64,
        share_code,
    }))
}
