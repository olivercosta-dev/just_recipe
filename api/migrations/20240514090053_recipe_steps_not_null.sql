-- Add migration script here
ALTER TABLE step
ALTER COLUMN step_id SET NOT NULL,
ALTER COLUMN recipe_id SET NOT NULL,
ALTER COLUMN step_number SET NOT NULL,
ALTER COLUMN instruction SET NOT NULL;