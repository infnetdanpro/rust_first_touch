-- Add migration script here
ALTER TABLE users ADD UNIQUE (email);