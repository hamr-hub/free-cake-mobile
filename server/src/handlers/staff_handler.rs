use axum::{extract::{State, Path, Query, Extension}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::AppState;
use crate::errors::AppError;
use crate::db::models::{Staff, AttendanceRecord};
use crate::services::audit_log::AuditLogService;
use crate::app_middleware::auth::Claims;

#[derive(Deserialize)]
pub struct CreateStaffRequest {
    pub store_id: i64,
    pub name: String,
    pub phone: String,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct ListStaffQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub store_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct StaffListResponse {
    pub list: Vec<Staff>,
    pub total: i64,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateStaffRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.name.is_empty() || req.phone.is_empty() {
        return Err(AppError::BadRequest("Name and phone are required".into()));
    }

    let store_exists = sqlx::query("SELECT id FROM store WHERE id = $1 AND status = 'active'")
        .bind(req.store_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if store_exists.is_none() {
        return Err(AppError::BadRequest("Store not found or inactive".into()));
    }

    let role = req.role.unwrap_or_else(|| "operator".to_string());
    let row = sqlx::query(
        "INSERT INTO staff (store_id, name, phone, role, status) VALUES ($1, $2, $3, $4, 'active') RETURNING id"
    )
    .bind(req.store_id)
    .bind(&req.name)
    .bind(&req.phone)
    .bind(&role)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let staff_id = row.get::<i64, _>("id");
    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "staff_created", "staff", staff_id, &format!("name={}, store_id={}", req.name, req.store_id)).await;

    Ok(Json(serde_json::json!({
        "id": staff_id,
        "status": "active"
    })))
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListStaffQuery>,
) -> Result<Json<StaffListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM staff WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM staff WHERE 1=1");

    if let Some(store_id) = params.store_id {
        query_builder.push(" AND store_id = ");
        query_builder.push_bind(store_id);
        count_builder.push(" AND store_id = ");
        count_builder.push_bind(store_id);
    }
    if let Some(ref status) = params.status {
        query_builder.push(" AND status = ");
        query_builder.push_bind(status);
        count_builder.push(" AND status = ");
        count_builder.push_bind(status);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<Staff>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(StaffListResponse { list, total }))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Staff>, AppError> {
    let staff = sqlx::query_as::<_, Staff>("SELECT * FROM staff WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Staff not found".into()))?;

    Ok(Json(staff))
}

pub async fn update_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateStaffStatusRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.new_status != "active" && req.new_status != "inactive" {
        return Err(AppError::BadRequest("Invalid status".into()));
    }

    let exists = sqlx::query("SELECT id FROM staff WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if exists.is_none() {
        return Err(AppError::NotFound("Staff not found".into()));
    }

    sqlx::query("UPDATE staff SET status = $1 WHERE id = $2")
        .bind(&req.new_status)
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "staff_status_update", "staff", id, &format!("new_status={}", req.new_status)).await;

    Ok(Json(serde_json::json!({ "id": id, "status": req.new_status })))
}

#[derive(Deserialize)]
pub struct UpdateStaffStatusRequest {
    pub new_status: String,
}

// ---- Attendance API ----

#[derive(Deserialize)]
pub struct CheckInRequest {
    pub staff_id: i64,
    pub store_id: i64,
}

#[derive(Serialize)]
pub struct AttendanceResponse {
    pub record_id: i64,
    pub status: String,
}

pub async fn check_in(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CheckInRequest>,
) -> Result<Json<AttendanceResponse>, AppError> {
    let staff = sqlx::query("SELECT id, store_id, status FROM staff WHERE id = $1")
        .bind(req.staff_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Staff not found".into()))?;

    let staff_status: String = staff.get("status");
    if staff_status != "active" {
        return Err(AppError::BadRequest("Staff is not active".into()));
    }

    let staff_store_id: i64 = staff.get("store_id");
    if staff_store_id != req.store_id {
        return Err(AppError::BadRequest("Staff does not belong to this store".into()));
    }

    // Check if already checked in today without checkout
    let existing = sqlx::query(
        "SELECT id FROM attendance_record WHERE staff_id = $1 AND store_id = $2 AND check_in_at >= CURRENT_DATE AND check_out_at IS NULL"
    )
    .bind(req.staff_id)
    .bind(req.store_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    if existing.is_some() {
        return Err(AppError::Conflict("Already checked in today".into()));
    }

    let row = sqlx::query(
        "INSERT INTO attendance_record (staff_id, store_id, check_in_at, status) VALUES ($1, $2, CURRENT_TIMESTAMP, 'normal') RETURNING id"
    )
    .bind(req.staff_id)
    .bind(req.store_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let record_id = row.get::<i64, _>("id");
    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "staff_check_in", "attendance_record", record_id, &format!("staff_id={}, store_id={}", req.staff_id, req.store_id)).await;

    Ok(Json(AttendanceResponse {
        record_id,
        status: "checked_in".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct CheckOutRequest {
    pub staff_id: i64,
    pub store_id: i64,
}

pub async fn check_out(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CheckOutRequest>,
) -> Result<Json<AttendanceResponse>, AppError> {
    let record = sqlx::query(
        "SELECT id, status FROM attendance_record WHERE staff_id = $1 AND store_id = $2 AND check_in_at >= CURRENT_DATE AND check_out_at IS NULL"
    )
    .bind(req.staff_id)
    .bind(req.store_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
    .ok_or_else(|| AppError::NotFound("No active check-in record found for today".into()))?;

    let record_id: i64 = record.get("id");

    sqlx::query("UPDATE attendance_record SET check_out_at = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(record_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    AuditLogService::log_with_pool(&state.db_pool, claims.user_id, "staff_check_out", "attendance_record", record_id, &format!("staff_id={}, store_id={}", req.staff_id, req.store_id)).await;

    Ok(Json(AttendanceResponse {
        record_id,
        status: "checked_out".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct ListAttendanceQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub staff_id: Option<i64>,
    pub store_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct AttendanceListResponse {
    pub list: Vec<AttendanceRecord>,
    pub total: i64,
}

pub async fn list_attendance(
    State(state): State<AppState>,
    Query(params): Query<ListAttendanceQuery>,
) -> Result<Json<AttendanceListResponse>, AppError> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM attendance_record WHERE 1=1");
    let mut count_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("SELECT COUNT(*) FROM attendance_record WHERE 1=1");

    if let Some(staff_id) = params.staff_id {
        query_builder.push(" AND staff_id = ");
        query_builder.push_bind(staff_id);
        count_builder.push(" AND staff_id = ");
        count_builder.push_bind(staff_id);
    }
    if let Some(store_id) = params.store_id {
        query_builder.push(" AND store_id = ");
        query_builder.push_bind(store_id);
        count_builder.push(" AND store_id = ");
        count_builder.push_bind(store_id);
    }
    if let Some(ref status) = params.status {
        query_builder.push(" AND status = ");
        query_builder.push_bind(status);
        count_builder.push(" AND status = ");
        count_builder.push_bind(status);
    }

    query_builder.push(" ORDER BY created_at DESC LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let total = count_builder.build_query_scalar::<i64>()
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let list = query_builder.build_query_as::<AttendanceRecord>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(AttendanceListResponse { list, total }))
}
