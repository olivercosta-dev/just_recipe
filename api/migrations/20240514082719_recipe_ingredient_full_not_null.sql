-- Add migration script here
ALTER TABLE recipe_ingredient
ALTER COLUMN recipe_id SET NOT NULL,
ALTER COLUMN unit_id SET NOT NULL,
ALTER COLUMN ingredient_id SET NOT NULL;
