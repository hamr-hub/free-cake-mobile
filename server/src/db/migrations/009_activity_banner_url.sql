-- Add banner_url column to activity table
ALTER TABLE activity ADD COLUMN IF NOT EXISTS banner_url TEXT;
