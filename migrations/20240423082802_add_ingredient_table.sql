-- Add migration script here
CREATE TABLE IF NOT EXISTS ingredient (
    ingredient_id SERIAL PRIMARY KEY, 
    singular_name VARCHAR(100) NOT NULL,
    plural_name VARCHAR(101) NOT NULL
);