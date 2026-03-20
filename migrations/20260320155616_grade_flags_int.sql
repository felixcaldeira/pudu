-- Add migration script here
ALTER TABLE modules MODIFY grade_flags INT UNSIGNED NOT NULL DEFAULT 0;