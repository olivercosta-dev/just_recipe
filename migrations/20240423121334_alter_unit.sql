DO $$
BEGIN
    -- Check if the 'name' column exists before renaming it
    IF EXISTS (SELECT 1 FROM information_schema.columns
               WHERE table_name='unit' AND column_name='name') THEN
        ALTER TABLE unit RENAME COLUMN name TO singular_name;
    END IF;

    -- Check if the 'plural_name' column does not exist before adding it
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name='unit' AND column_name='plural_name') THEN
        ALTER TABLE unit ADD COLUMN plural_name VARCHAR(50) NOT NULL UNIQUE;
    END IF;
END $$;
