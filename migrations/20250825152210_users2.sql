-- Add migration script here
CREATE TABLE users (id serial primary key, email VARCHAR(255), password VARCHAR(255));