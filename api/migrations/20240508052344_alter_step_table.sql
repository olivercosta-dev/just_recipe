-- Step 1: Drop the existing foreign key constraint
ALTER TABLE step
DROP CONSTRAINT step_recipe_id_fkey;

-- Step 2: Add the foreign key constraint with ON DELETE CASCADE
ALTER TABLE step
ADD FOREIGN KEY (recipe_id) REFERENCES recipe(recipe_id) ON DELETE CASCADE;
