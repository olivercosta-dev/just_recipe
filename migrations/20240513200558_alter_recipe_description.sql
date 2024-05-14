-- Add migration script here
ALTER TABLE recipe
ALTER COLUMN description 
SET NOT NULL;