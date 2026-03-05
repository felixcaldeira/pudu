-- Add migration script here
ALTER TABLE users MODIFY flags INT NOT NULL DEFAULT 0;