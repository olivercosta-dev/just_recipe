-- Add migration script here
ALTER TABLE recipe_ingredient
ALTER COLUMN quantity 
SET NOT NULL;