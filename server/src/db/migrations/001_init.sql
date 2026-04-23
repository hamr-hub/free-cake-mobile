-- Free Cake MVP Database Schema

CREATE TABLE IF NOT EXISTS region (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    coverage_radius_km INT NOT NULL DEFAULT 10,
    center_lat DOUBLE NOT NULL,
    center_lng DOUBLE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS store (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    region_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    address VARCHAR(255) NOT NULL,
    lat DOUBLE NOT NULL,
    lng DOUBLE NOT NULL,
    daily_capacity INT NOT NULL DEFAULT 100,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    contact_name VARCHAR(50) NOT NULL DEFAULT '',
    contact_phone VARCHAR(20) NOT NULL DEFAULT '',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_region_id (region_id)
);

CREATE TABLE IF NOT EXISTS user (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    phone VARCHAR(20) NOT NULL,
    phone_hash VARCHAR(64) NOT NULL,
    open_id VARCHAR(100) NOT NULL DEFAULT '',
    nickname VARCHAR(50) NOT NULL DEFAULT '',
    region_id BIGINT,
    role VARCHAR(20) NOT NULL DEFAULT 'user',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE INDEX idx_phone (phone),
    INDEX idx_region_id (region_id)
);

CREATE TABLE IF NOT EXISTS user_identity (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    identity_type VARCHAR(20) NOT NULL,
    identity_value VARCHAR(255) NOT NULL,
    device_id VARCHAR(100) NOT NULL DEFAULT '',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_user_id (user_id),
    INDEX idx_identity (identity_type, identity_value)
);

CREATE TABLE IF NOT EXISTS activity (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    region_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    registration_start_at DATETIME NOT NULL,
    registration_end_at DATETIME NOT NULL,
    voting_start_at DATETIME NOT NULL,
    voting_end_at DATETIME NOT NULL,
    max_winner_count INT NOT NULL DEFAULT 100,
    status VARCHAR(20) NOT NULL DEFAULT 'draft',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_region_id (region_id),
    INDEX idx_status (status)
);

CREATE TABLE IF NOT EXISTS activity_rule (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    activity_id BIGINT NOT NULL,
    max_votes_per_day INT NOT NULL DEFAULT 3,
    cake_size VARCHAR(10) NOT NULL DEFAULT '6inch',
    cream_type VARCHAR(20) NOT NULL DEFAULT 'animal',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_activity_id (activity_id)
);

CREATE TABLE IF NOT EXISTS design_template (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    image_url VARCHAR(500) NOT NULL,
    cake_size VARCHAR(10) NOT NULL DEFAULT '6inch',
    cream_type VARCHAR(20) NOT NULL DEFAULT 'animal',
    decoration_params TEXT,
    producible_level VARCHAR(20) NOT NULL DEFAULT 'standard',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ai_generation_record (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
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
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_user_activity (user_id, activity_id)
);

CREATE TABLE IF NOT EXISTS contest_entry (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    activity_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    selected_generation_id BIGINT NOT NULL,
    selected_template_id BIGINT NOT NULL,
    title VARCHAR(100) NOT NULL,
    share_code VARCHAR(20) NOT NULL UNIQUE,
    image_url VARCHAR(500) NOT NULL,
    raw_vote_count INT NOT NULL DEFAULT 0,
    valid_vote_count INT NOT NULL DEFAULT 0,
    risk_score DOUBLE NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_activity_votes (activity_id, valid_vote_count DESC),
    INDEX idx_user_activity (user_id, activity_id),
    INDEX idx_share_code (share_code)
);

CREATE TABLE IF NOT EXISTS vote_record (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
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
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_activity_entry (activity_id, entry_id, created_at),
    INDEX idx_voter_open (voter_open_id, activity_id),
    INDEX idx_voter_device (voter_device_id, activity_id)
);

CREATE TABLE IF NOT EXISTS risk_event (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    activity_id BIGINT NOT NULL,
    entry_id BIGINT NOT NULL,
    risk_type VARCHAR(50) NOT NULL,
    risk_level VARCHAR(20) NOT NULL DEFAULT 'medium',
    description TEXT,
    related_user_ids TEXT,
    device_ids TEXT,
    ip_list TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'open',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at DATETIME NULL,
    INDEX idx_activity_status (activity_id, status)
);

CREATE TABLE IF NOT EXISTS winner_record (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    activity_id BIGINT NOT NULL,
    entry_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    rank INT NOT NULL,
    valid_vote_count INT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'confirmed',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_activity (activity_id)
);

CREATE TABLE IF NOT EXISTS reward_order (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    winner_id BIGINT NOT NULL,
    store_id BIGINT NOT NULL,
    order_type VARCHAR(20) NOT NULL DEFAULT 'free',
    template_id BIGINT NOT NULL,
    scheduled_date DATETIME NULL,
    production_status VARCHAR(20) NOT NULL DEFAULT 'pending',
    redeem_status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_store_date (store_id, scheduled_date, production_status)
);

CREATE TABLE IF NOT EXISTS production_batch (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    store_id BIGINT NOT NULL,
    activity_id BIGINT NOT NULL,
    scheduled_date DATETIME NOT NULL,
    total_count INT NOT NULL DEFAULT 0,
    completed_count INT NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS production_task (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    batch_id BIGINT NOT NULL,
    order_id BIGINT NOT NULL,
    store_id BIGINT NOT NULL,
    template_id BIGINT NOT NULL,
    device_task_payload TEXT,
    task_status VARCHAR(20) NOT NULL DEFAULT 'pending',
    started_at DATETIME NULL,
    completed_at DATETIME NULL,
    fail_reason VARCHAR(500) NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_batch (batch_id)
);

CREATE TABLE IF NOT EXISTS redeem_code (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    order_id BIGINT NOT NULL,
    code VARCHAR(20) NOT NULL UNIQUE,
    expires_at DATETIME NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'valid',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_code (code),
    INDEX idx_order (order_id)
);

CREATE TABLE IF NOT EXISTS redeem_record (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    order_id BIGINT NOT NULL,
    redeem_code_id BIGINT NOT NULL,
    store_id BIGINT NOT NULL,
    verifier_staff_id BIGINT NOT NULL,
    redeem_result VARCHAR(20) NOT NULL DEFAULT 'success',
    redeem_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_store_redeem (store_id, redeem_at)
);

CREATE TABLE IF NOT EXISTS inventory_item (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    store_id BIGINT NOT NULL,
    name VARCHAR(100) NOT NULL,
    category VARCHAR(50) NOT NULL DEFAULT '',
    unit VARCHAR(20) NOT NULL DEFAULT '',
    quantity DOUBLE NOT NULL DEFAULT 0,
    safety_threshold DOUBLE NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_store (store_id)
);

CREATE TABLE IF NOT EXISTS inventory_txn (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    store_id BIGINT NOT NULL,
    item_id BIGINT NOT NULL,
    txn_type VARCHAR(20) NOT NULL,
    quantity DOUBLE NOT NULL,
    reason VARCHAR(200) NOT NULL DEFAULT '',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_store_item (store_id, item_id, created_at)
);

CREATE TABLE IF NOT EXISTS staff (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    store_id BIGINT NOT NULL,
    name VARCHAR(50) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'operator',
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_store (store_id)
);

CREATE TABLE IF NOT EXISTS attendance_record (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    staff_id BIGINT NOT NULL,
    store_id BIGINT NOT NULL,
    check_in_at DATETIME NULL,
    check_out_at DATETIME NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'normal',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_staff_date (staff_id, check_in_at)
);

CREATE TABLE IF NOT EXISTS audit_log (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    operator_id BIGINT NOT NULL,
    action VARCHAR(50) NOT NULL,
    target_type VARCHAR(50) NOT NULL,
    target_id BIGINT NOT NULL,
    detail TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_target (target_type, target_id),
    INDEX idx_operator (operator_id)
);
