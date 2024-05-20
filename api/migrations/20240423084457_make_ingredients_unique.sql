-- Add migration script here
-- ALTER TABLE ingredient
-- ADD CONSTRAINT unique_singular_name UNIQUE (singular_name),
-- ADD CONSTRAINT unique_plural_name UNIQUE (plural_name);
DO $$
BEGIN
    -- Check for unique_singular_name constraint
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'unique_singular_name') THEN
        ALTER TABLE ingredient ADD CONSTRAINT unique_singular_name UNIQUE (singular_name);
    END IF;
    
    -- Check for unique_plural_name constraint
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'unique_plural_name') THEN
        ALTER TABLE ingredient ADD CONSTRAINT unique_plural_name UNIQUE (plural_name);
    END IF;
END $$;
