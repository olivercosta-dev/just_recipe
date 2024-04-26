-- Rename a column and add a new column
ALTER TABLE unit
RENAME COLUMN name TO singular_name;

ALTER TABLE unit
ADD COLUMN plural_name VARCHAR(50) NOT NULL UNIQUE;
