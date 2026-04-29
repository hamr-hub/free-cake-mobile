-- Add decoration_params column to activity_rule for AI generation rate limit and other rule metadata
ALTER TABLE activity_rule ADD COLUMN IF NOT EXISTS decoration_params JSONB DEFAULT '{}';
