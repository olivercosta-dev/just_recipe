-- Add migration script here
ALTER TABLE ingredient
ADD CONSTRAINT unique_singular_name UNIQUE (singular_name),
ADD CONSTRAINT unique_plural_name UNIQUE (plural_name);
