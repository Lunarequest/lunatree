-- Your SQL goes here
CREATE TABLE users (
    userid SERIAL PRIMARY KEY,
    username varchar(50) UNIQUE,
    password varchar(300),
    email varchar(255),
    active bool
);

ALTER TABLE users
ALTER COLUMN active
SET DEFAULT false;
