-- Your SQL goes here
CREATE TABLE users (
    userid SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(300) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    active BOOL NOT NULL
);

ALTER TABLE users
ALTER COLUMN active
SET DEFAULT false;
