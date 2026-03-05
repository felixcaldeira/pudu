-- Add migration script here
ALTER TABLE users MODIFY flags INT UNSIGNED NOT NULL DEFAULT 0;