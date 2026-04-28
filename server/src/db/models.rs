#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Region {
    pub id: i64,
    pub name: String,
    pub province: String,
    pub city: String,
    pub coverage_radius_km: i32,
    pub center_lat: f64,
    pub center_lng: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Store {
    pub id: i64,
    pub region_id: i64,
    pub name: String,
    pub address: String,
    pub lat: f64,
    pub lng: f64,
    pub daily_capacity: i32,
    pub status: String,
    pub contact_name: String,
    pub contact_phone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AppUser {
    pub id: i64,
    pub phone: String,
    pub phone_hash: String,
    pub open_id: String,
    pub nickname: String,
    pub region_id: Option<i64>,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserIdentity {
    pub id: i64,
    pub user_id: i64,
    pub identity_type: String,
    pub identity_value: String,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Activity {
    pub id: i64,
    pub region_id: i64,
    pub name: String,
    pub registration_start_at: DateTime<Utc>,
    pub registration_end_at: DateTime<Utc>,
    pub voting_start_at: DateTime<Utc>,
    pub voting_end_at: DateTime<Utc>,
    pub max_winner_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AIGenerationRecord {
    pub id: i64,
    pub user_id: i64,
    pub activity_id: i64,
    pub scene: String,
    pub theme: String,
    pub blessing: String,
    pub color_preference: String,
    pub style: String,
    pub prompt: String,
    pub image_urls: String,
    pub template_ids: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct DesignTemplate {
    pub id: i64,
    pub name: String,
    pub image_url: String,
    pub cake_size: String,
    pub cream_type: String,
    pub decoration_params: String,
    pub producible_level: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ContestEntry {
    pub id: i64,
    pub activity_id: i64,
    pub user_id: i64,
    pub selected_generation_id: i64,
    pub selected_template_id: i64,
    pub title: String,
    pub share_code: String,
    pub image_url: String,
    pub raw_vote_count: i32,
    pub valid_vote_count: i32,
    pub risk_score: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct VoteRecord {
    pub id: i64,
    pub activity_id: i64,
    pub entry_id: i64,
    pub voter_user_id: i64,
    pub voter_open_id: String,
    pub voter_phone_hash: String,
    pub voter_device_id: String,
    pub ip: String,
    pub geohash: String,
    pub vote_status: String,
    pub risk_tags: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RiskEvent {
    pub id: i64,
    pub activity_id: i64,
    pub entry_id: i64,
    pub risk_type: String,
    pub risk_level: String,
    pub description: String,
    pub related_user_ids: String,
    pub device_ids: String,
    pub ip_list: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct WinnerRecord {
    pub id: i64,
    pub activity_id: i64,
    pub entry_id: i64,
    pub user_id: i64,
    pub rank: i32,
    pub valid_vote_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RewardOrder {
    pub id: i64,
    pub winner_id: i64,
    pub store_id: i64,
    pub order_type: String,
    pub template_id: i64,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub production_status: String,
    pub redeem_status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProductionBatch {
    pub id: i64,
    pub store_id: i64,
    pub activity_id: i64,
    pub scheduled_date: DateTime<Utc>,
    pub total_count: i32,
    pub completed_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProductionTask {
    pub id: i64,
    pub batch_id: i64,
    pub order_id: i64,
    pub store_id: i64,
    pub template_id: i64,
    pub device_task_payload: String,
    pub task_status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub fail_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RedeemCode {
    pub id: i64,
    pub order_id: i64,
    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RedeemRecord {
    pub id: i64,
    pub order_id: i64,
    pub redeem_code_id: i64,
    pub store_id: i64,
    pub verifier_staff_id: i64,
    pub redeem_result: String,
    pub redeem_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: i64,
    pub store_id: i64,
    pub name: String,
    pub category: String,
    pub unit: String,
    pub quantity: f64,
    pub safety_threshold: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct InventoryTxn {
    pub id: i64,
    pub store_id: i64,
    pub item_id: i64,
    pub txn_type: String,
    pub quantity: f64,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i64,
    pub operator_id: i64,
    pub action: String,
    pub target_type: String,
    pub target_id: i64,
    pub detail: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Staff {
    pub id: i64,
    pub store_id: i64,
    pub name: String,
    pub phone: String,
    pub role: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AttendanceRecord {
    pub id: i64,
    pub staff_id: i64,
    pub store_id: i64,
    pub check_in_at: Option<DateTime<Utc>>,
    pub check_out_at: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
