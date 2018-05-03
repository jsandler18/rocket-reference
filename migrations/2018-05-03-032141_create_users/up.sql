-- Your SQL goes here
CREATE TABLE users (
    id serial PRIMARY KEY,
    username TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    password_hash VARCHAR(256) NOT NULL
)
