-- Your SQL goes here
DELETE FROM users;
ALTER TABLE users ADD COLUMN username varchar(20) NOT NULL;
ALTER TABLE users ADD COLUMN password varchar(20) NOT NULL;
