-- Add migration script here
CREATE TABLE IF NOT EXISTS step (
    step_id SERIAL PRIMARY KEY,
    recipe_id INT,
    step_number INT,
    instruction TEXT,
    FOREIGN KEY (recipe_id) REFERENCES recipe(recipe_id),
    UNIQUE (recipe_id, step_number)
);