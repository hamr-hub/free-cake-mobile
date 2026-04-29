-- Add error_description column for production task error reporting
ALTER TABLE production_task ADD COLUMN IF NOT EXISTS error_description TEXT;

-- Add paused_at and cancelled_at timestamps
ALTER TABLE production_task ADD COLUMN IF NOT EXISTS paused_at TIMESTAMPTZ;
ALTER TABLE production_task ADD COLUMN IF NOT EXISTS cancelled_at TIMESTAMPTZ;
