-- 004: Payment support schema additions

-- Add payment columns to reward_order
ALTER TABLE reward_order
    ADD COLUMN IF NOT EXISTS order_type        VARCHAR(16)    DEFAULT 'free',
    ADD COLUMN IF NOT EXISTS amount            NUMERIC(10, 2) DEFAULT 0,
    ADD COLUMN IF NOT EXISTS pay_status         VARCHAR(20)     DEFAULT 'free',
    ADD COLUMN IF NOT EXISTS pay_transaction_id VARCHAR(64),
    ADD COLUMN IF NOT EXISTS paid_at            TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS closed_at          TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS refund_status      VARCHAR(20),
    ADD COLUMN IF NOT EXISTS refund_reason      TEXT,
    ADD COLUMN IF NOT EXISTS refund_txn_id      VARCHAR(64),
    ADD COLUMN IF NOT EXISTS refunded_at        TIMESTAMPTZ;

-- Price configuration per region and spec
CREATE TABLE IF NOT EXISTS price_config (
    id          BIGSERIAL PRIMARY KEY,
    region_id   BIGINT       NOT NULL REFERENCES region(id),
    cake_size   VARCHAR(16)  NOT NULL,  -- '6inch', '8inch', '10inch'
    cream_type  VARCHAR(32)  NOT NULL,  -- 'animal', 'vegetable', 'mixed'
    price       NUMERIC(10, 2) NOT NULL,
    status      VARCHAR(16)  DEFAULT 'active',
    created_at  TIMESTAMPTZ  DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  DEFAULT NOW(),
    UNIQUE (region_id, cake_size, cream_type)
);

-- Payment audit trail
CREATE TABLE IF NOT EXISTS payment_record (
    id              BIGSERIAL PRIMARY KEY,
    order_id        BIGINT       NOT NULL REFERENCES reward_order(id),
    transaction_id  VARCHAR(64),
    pay_channel     VARCHAR(32)  NOT NULL, -- 'wechat_h5', 'wechat_mini'
    amount          NUMERIC(10, 2) NOT NULL,
    status          VARCHAR(20)  NOT NULL, -- 'pending', 'success', 'failed', 'refunded'
    raw_response    JSONB,
    created_at      TIMESTAMPTZ  DEFAULT NOW()
);

-- Add updated_at to design_template
ALTER TABLE design_template
    ADD COLUMN IF NOT EXISTS status     VARCHAR(16) DEFAULT 'active',
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();

-- Auto-update trigger for design_template
CREATE OR REPLACE FUNCTION update_design_template_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_design_template_updated_at ON design_template;
CREATE TRIGGER set_design_template_updated_at
    BEFORE UPDATE ON design_template
    FOR EACH ROW EXECUTE FUNCTION update_design_template_updated_at();

-- Auto-update trigger for price_config
CREATE OR REPLACE FUNCTION update_price_config_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_price_config_updated_at ON price_config;
CREATE TRIGGER set_price_config_updated_at
    BEFORE UPDATE ON price_config
    FOR EACH ROW EXECUTE FUNCTION update_price_config_updated_at();
