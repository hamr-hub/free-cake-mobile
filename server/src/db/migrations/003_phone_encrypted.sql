-- Add phone_encrypted column for AES-256-GCM encrypted phone storage
ALTER TABLE app_user ADD COLUMN IF NOT EXISTS phone_encrypted TEXT NOT NULL DEFAULT '';

-- Add index for phone_hash lookups (already exists via phone column unique index,
-- but phone_hash should be the primary lookup method going forward)
CREATE INDEX IF NOT EXISTS idx_app_user_phone_hash ON app_user (phone_hash);
