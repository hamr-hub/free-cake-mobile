use axum::{extract::{State, Path, Extension, Query}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::DesignTemplate;
use crate::services::audit_log::AuditLogService;
use crate::app_middleware::auth::Claims;
use crate::services::validation;

#[derive(Deserialize)]
pub struct TemplateListQuery {
    pub producible_level: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
pub struct TemplateListResponse {
    pub list: Vec<DesignTemplate>,
    pub total: i64,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<TemplateListQuery>,
) -> Result<Json<TemplateListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM design_template WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM design_template WHERE 1=1");

    if let Some(ref pl) = params.producible_level {
        query_builder.push(" AND producible_level = "); query_builder.push_bind(pl);
        count_builder.push(" AND producible_level = "); count_builder.push_bind(pl);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT "); query_builder.push_bind(page_size);
    query_builder.push(" OFFSET "); query_builder.push_bind(offset);

    let total: i64 = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<DesignTemplate>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(TemplateListResponse { list, total }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<DesignTemplate>, AppError> {
    let template = sqlx::query_as::<_, DesignTemplate>("SELECT * FROM design_template WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let t = template.ok_or_else(|| AppError::NotFound("Template not found".into()))?;
    Ok(Json(t))
}

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub image_url: String,
    pub cake_size: String,
    pub cream_type: String,
    pub decoration_params: String,
    pub producible_level: String,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateTemplateRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    validation::validate_string_max(&req.name, 100, "Template name")?;
    validation::validate_cake_size(&req.cake_size)?;
    validation::validate_cream_type(&req.cream_type)?;
    let row = sqlx::query(
        "INSERT INTO design_template (name, image_url, cake_size, cream_type, decoration_params, producible_level) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
    )
    .bind(&req.name)
    .bind(&req.image_url)
    .bind(&req.cake_size)
    .bind(&req.cream_type)
    .bind(&req.decoration_params)
    .bind(&req.producible_level)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let id: i64 = row.get("id");
    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "create", "design_template", id, &format!("name={}", req.name)).await;

    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub cake_size: Option<String>,
    pub cream_type: Option<String>,
    pub decoration_params: Option<String>,
    pub producible_level: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTemplateRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE design_template SET id = id");
    if let Some(ref v) = req.name { builder.push(", name = "); builder.push_bind(v); }
    if let Some(ref v) = req.image_url { builder.push(", image_url = "); builder.push_bind(v); }
    if let Some(ref v) = req.cake_size { builder.push(", cake_size = "); builder.push_bind(v); }
    if let Some(ref v) = req.cream_type { builder.push(", cream_type = "); builder.push_bind(v); }
    if let Some(ref v) = req.decoration_params { builder.push(", decoration_params = "); builder.push_bind(v); }
    if let Some(ref v) = req.producible_level { builder.push(", producible_level = "); builder.push_bind(v); }
    builder.push(" WHERE id = "); builder.push_bind(id);

    builder.build().execute(&state.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "update", "design_template", id, "updated template").await;

    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)]
pub struct UpdateTemplateStatusRequest {
    pub status: String,
}

pub async fn update_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTemplateStatusRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.status != "active" && req.status != "inactive" {
        return Err(AppError::BadRequest("Status must be active or inactive".into()));
    }
    sqlx::query("UPDATE design_template SET status = $1 WHERE id = $2")
        .bind(&req.status)
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "template_status_update", "design_template", id, &format!("status={}", req.status)).await;

    Ok(Json(serde_json::json!({ "id": id, "status": req.status })))
}
