-- Add migration script here
CREATE TABLE IF NOT EXISTS unit (
    unit_id SERIAL PRIMARY KEY,
    name varchar(50) NOT NULL UNIQUE
);