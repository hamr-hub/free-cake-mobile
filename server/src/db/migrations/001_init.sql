-- Free Cake MVP Database Schema (PostgreSQL / Supabase)

-- updated_at auto-update trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS region (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    province VARCHAR(50) NOT NULL DEFAULT '',
    city VARCHAR(50) NOT NULL DEFAULT '',
    coverage_radius_km INTEGER NOT NULL DEFAULT 10,
    center_lat DOUBLE PRECISION NOT NULL,
    center_lng DOUBLE PRECISION NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER trg_region_updated_at BEFORE UPDATE ON region FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS store (
    id BIGSERIAL PRIMARY KEY,
    region_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    address VARCHAR(255) NOT NULL,
    lat DOUBLE PRECISION NOT NULL,
    lng DOUBLE PRECISION NOT NULL,
    daily_capacity INTEGER NOT NULL DEFAULT 100,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    contact_name VARCHAR(50) NOT NULL DEFAULT '',
    contact_phone VARCHAR(20) NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_store_region_id ON store (region_id);
CREATE TRIGGER trg_store_updated_at BEFORE UPDATE ON store FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS app_user (
    id BIGSERIAL PRIMARY KEY,
    phone VARCHAR(20) NOT NULL,
    phone_hash VARCHAR(64) NOT NULL,
    open_id VARCHAR(100) NOT NULL DEFAULT '',
    nickname VARCHAR(50) NOT NULL DEFAULT '',
    region_id BIGINT,
    role VARCHAR(20) NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_app_user_phone ON app_user (phone);
CREATE INDEX idx_app_user_region_id ON app_user (region_id);
CREATE TRIGGER trg_app_user_updated_at BEFORE UPDATE ON app_user FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS user_identity (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    identity_type VARCHAR(20) NOT NULL,
    identity_value VARCHAR(255) NOT NULL,
    device_id VARCHAR(100) NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_user_identity_user_id ON user_identity (user_id);
CREATE INDEX idx_user_identity_type_value ON user_identity (identity_type, identity_value);

CREATE TABLE IF NOT EXISTS activity (
    id BIGSERIAL PRIMARY KEY,
    region_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    registration_start_at TIMESTAMPTZ NOT NULL,
    registration_end_at TIMESTAMPTZ NOT NULL,
    voting_start_at TIMESTAMPTZ NOT NULL,
    voting_end_at TIMESTAMPTZ NOT NULL,
    max_winner_count INTEGER NOT NULL DEFAULT 100,
    status VARCHAR(20) NOT NULL DEFAULT 'draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_activity_region_id ON activity (region_id);
CREATE INDEX idx_activity_status ON activity (status);
CREATE TRIGGER trg_activity_updated_at BEFORE UPDATE ON activity FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS activity_rule (
    id BIGSERIAL PRIMARY KEY,
    activity_id BIGINT NOT NULL,
    max_votes_per_day INTEGER NOT NULL DEFAULT 3,
    cake_size VARCHAR(10) NOT NULL DEFAULT '6inch',
    cream_type VARCHAR(20) NOT NULL DEFAULT 'animal',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_activity_rule_activity_id ON activity_rule (activity_id);

CREATE TABLE IF NOT EXISTS design_template (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    image_url VARCHAR(500) NOT NULL,
    cake_size VARCHAR(10) NOT NULL DEFAULT '6inch',
    cream_type VARCHAR(20) NOT NULL DEFAULT 'animal',
    decoration_params TEXT,
    producible_level VARCHAR(20) NOT NULL DEFAULT 'standard',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ai_generation_record (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    activity_id BIGINT NOT NULL,
    scene VARCHAR(20) NOT NULL,
    theme VARCHAR(100) NOT NULL,
    blessing VARCHAR(200) NOT NULL DEFAULT '',
    color_preference VARCHAR(50) NOT NULL DEFAULT '',
    style VARCHAR(50) NOT NULL DEFAULT '',
    prompt TEXT,
    image_urls TEXT,
    template_ids TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_ai_generation_user_activity ON ai_generation_record (user_id, activity_id);

CREATE TABLE IF NOT EXISTS contest_entry (
    id BIGSERIAL PRIMARY KEY,
    activity_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    selected_generation_id BIGINT NOT NULL,
    selected_template_id BIGINT NOT NULL,
    title VARCHAR(100) NOT NULL,
    share_code VARCHAR(20) NOT NULL UNIQUE,
    image_url VARCHAR(500) NOT NULL,
    raw_vote_count INTEGER NOT NULL DEFAULT 0,
    valid_vote_count INTEGER NOT NULL DEFAULT 0,
    risk_score DOUBLE PRECISION NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_contest_entry_activity_votes ON contest_entry (activity_id, valid_vote_count DESC);
CREATE INDEX idx_contest_entry_user_activity ON contest_entry (user_id, activity_id);
CREATE TRIGGER trg_contest_entry_updated_at BEFORE UPDATE ON contest_entry FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS vote_record (
    id BIGSERIAL PRIMARY KEY,
    activity_id BIGINT NOT NULL,
    entry_id BIGINT NOT NULL,
    voter_user_id BIGINT NOT NULL,
    voter_open_id VARCHAR(100) NOT NULL DEFAULT '',
    voter_phone_hash VARCHAR(64) NOT NULL DEFAULT '',
    voter_device_id VARCHAR(100) NOT NULL DEFAULT '',
    ip VARCHAR(45) NOT NULL DEFAULT '',
    geohash VARCHAR(20) NOT NULL DEFAULT '',
    vote_status VARCHAR(20) NOT NULL DEFAULT 'valid',
    risk_tags VARCHAR(200) NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_vote_record_activity_entry ON vote_record (activity_id, entry_id, created_at);
CREATE INDEX idx_vote_record_voter_open ON vote_record (voter_open_id, activity_id);
CREATE INDEX idx_vote_record_voter_device ON vote_record (voter_device_id, activity_id);

CREATE TABLE IF NOT EXISTS risk_event (
    id BIGSERIAL PRIMARY KEY,
    activity_id BIGINT NOT NULL,
    entry_id BIGINT NOT NULL,
    risk_type VARCHAR(50) NOT NULL,
    risk_level VARCHAR(20) NOT NULL DEFAULT 'medium',
    description TEXT,
    related_user_ids TEXT,
    device_ids TEXT,
    ip_list TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMPTZ NULL
);
CREATE INDEX idx_risk_event_activity_status ON risk_event (activity_id, status);

CREATE TABLE IF NOT EXISTS winner_record (
    id BIGSERIAL PRIMARY KEY,
    activity_id BIGINT NOT NULL,
    entry_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    rank INTEGER NOT NULL,
    valid_vote_count INTEGER NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'confirmed',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_winner_record_activity ON winner_record (activity_id);

CREATE TABLE IF NOT EXISTS reward_order (
    id BIGSERIAL PRIMARY KEY,
    winner_id BIGINT NOT NULL,
    store_id BIGINT NOT NULL,
    order_type VARCHAR(20) NOT NULL DEFAULT 'free',
    template_id BIGINT NOT NULL,
    scheduled_date TIMESTAMPTZ NULL,
    production_status VARCHAR(20) NOT NULL DEFAULT 'pending',
    redeem_status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_reward_order_store_date ON reward_order (store_id, scheduled_date, production_status);
CREATE TRIGGER trg_reward_order_updated_at BEFORE UPDATE ON reward_order FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS production_batch (
    id BIGSERIAL PRIMARY KEY,
    store_id BIGINT NOT NULL,
    activity_id BIGINT NOT NULL,
    scheduled_date TIMESTAMPTZ NOT NULL,
    total_count INTEGER NOT NULL DEFAULT 0,
    completed_count INTEGER NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS production_task (
    id BIGSERIAL PRIMARY KEY,
    batch_id BIGINT NOT NULL,
    order_id BIGINT NOT NULL,
    store_id BIGINT NOT NULL,
    template_id BIGINT NOT NULL,
    device_task_payload TEXT,
    task_status VARCHAR(20) NOT NULL DEFAULT 'pending',
    started_at TIMESTAMPTZ NULL,
    completed_at TIMESTAMPTZ NULL,
    fail_reason VARCHAR(500) NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_production_task_batch ON production_task (batch_id);

CREATE TABLE IF NOT EXISTS redeem_code (
    id BIGSERIAL PRIMARY KEY,
    order_id BIGINT NOT NULL,
    code VARCHAR(20) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'valid',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_redeem_code_code ON redeem_code (code);
CREATE INDEX idx_redeem_code_order ON redeem_code (order_id);

CREATE TABLE IF NOT EXISTS redeem_record (
    id BIGSERIAL PRIMARY KEY,
    order_id BIGINT NOT NULL,
    redeem_code_id BIGINT NOT NULL,
    store_id BIGINT NOT NULL,
    verifier_staff_id BIGINT NOT NULL,
    redeem_result VARCHAR(20) NOT NULL DEFAULT 'success',
    redeem_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_redeem_record_store_redeem ON redeem_record (store_id, redeem_at);

CREATE TABLE IF NOT EXISTS inventory_item (
    id BIGSERIAL PRIMARY KEY,
    store_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    category VARCHAR(50) NOT NULL DEFAULT '',
    unit VARCHAR(20) NOT NULL DEFAULT '',
    quantity DOUBLE PRECISION NOT NULL DEFAULT 0,
    safety_threshold DOUBLE PRECISION NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_inventory_item_store ON inventory_item (store_id);
CREATE TRIGGER trg_inventory_item_updated_at BEFORE UPDATE ON inventory_item FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS inventory_txn (
    id BIGSERIAL PRIMARY KEY,
    store_id BIGINT NOT NULL,
    item_id BIGINT NOT NULL,
    txn_type VARCHAR(20) NOT NULL,
    quantity DOUBLE PRECISION NOT NULL,
    reason VARCHAR(200) NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_inventory_txn_store_item ON inventory_txn (store_id, item_id, created_at);

CREATE TABLE IF NOT EXISTS staff (
    id BIGSERIAL PRIMARY KEY,
    store_id BIGINT NOT NULL,
    name VARCHAR(50) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'operator',
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_staff_store ON staff (store_id);
CREATE TRIGGER trg_staff_updated_at BEFORE UPDATE ON staff FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS attendance_record (
    id BIGSERIAL PRIMARY KEY,
    staff_id BIGINT NOT NULL,
    store_id BIGINT NOT NULL,
    check_in_at TIMESTAMPTZ NULL,
    check_out_at TIMESTAMPTZ NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'normal',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_attendance_record_staff_date ON attendance_record (staff_id, check_in_at);

CREATE TABLE IF NOT EXISTS audit_log (
    id BIGSERIAL PRIMARY KEY,
    operator_id BIGINT NOT NULL,
    action VARCHAR(50) NOT NULL,
    target_type VARCHAR(50) NOT NULL,
    target_id BIGINT NOT NULL,
    detail TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_audit_log_target ON audit_log (target_type, target_id);
CREATE INDEX idx_audit_log_operator ON audit_log (operator_id);
