CREATE TABLE IF NOT EXISTS recipe_ingredient (
    recipe_id INT,
    ingredient_id INT,
    unit_id INT,
    quantity VARCHAR(50),
    PRIMARY KEY (recipe_id, ingredient_id),
    FOREIGN KEY (recipe_id) REFERENCES recipe(recipe_id) ON DELETE CASCADE,
    FOREIGN KEY (ingredient_id) REFERENCES ingredient(ingredient_id) ON DELETE RESTRICT,
    FOREIGN KEY (unit_id) REFERENCES unit(unit_id) ON DELETE RESTRICT
);