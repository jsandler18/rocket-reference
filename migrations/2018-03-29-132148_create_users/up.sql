-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    firstname VARCHAR(20) NOT NULL,
    lastname VARCHAR(20) NOT NULL,
    birthday DATE NOT NULL
)
