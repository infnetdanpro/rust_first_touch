-- Add migration script here
CREATE TABLE links (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER REFERENCES users(id) NOT NULL,
    short_code TEXT NOT NULL UNIQUE,
    destination_url TEXT NOT NULL,
    views INTEGER DEFAULT 0
)