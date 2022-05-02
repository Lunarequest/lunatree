-- Your SQL goes here
CREATE TABLE users (
    userid SERIAL PRIMARY KEY,
    username varchar(50) NOT NULL,
    password varchar(300) NOT NULL,
    email varchar(255) UNIQUE NOT NULL,
    active bool NOT NULL
);

ALTER TABLE users
ALTER COLUMN active
SET DEFAULT false;
