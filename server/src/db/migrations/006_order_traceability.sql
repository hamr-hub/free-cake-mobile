-- Add entry_id and user_id to reward_order for idempotency and traceability
ALTER TABLE reward_order
    ADD COLUMN IF NOT EXISTS entry_id BIGINT,
    ADD COLUMN IF NOT EXISTS user_id   BIGINT;

CREATE INDEX IF NOT EXISTS idx_reward_order_entry_id ON reward_order (entry_id);
CREATE INDEX IF NOT EXISTS idx_reward_order_user_id  ON reward_order (user_id);
