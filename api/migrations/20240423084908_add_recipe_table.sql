-- Add migration script here
CREATE TABLE IF NOT EXISTS recipe (
    recipe_id SERIAL PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    description TEXT
);