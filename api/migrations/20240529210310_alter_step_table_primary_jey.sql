-- Drop the existing step_id column
ALTER TABLE step
DROP COLUMN step_id;

-- Add the step_id column with SERIAL type and set it as the primary key
ALTER TABLE step
ADD COLUMN step_id SERIAL PRIMARY KEY;

