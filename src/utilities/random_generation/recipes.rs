use std::collections::HashSet;

use fake::{Fake, Faker};
use sqlx::PgPool;

use crate::{
    ingredient::Ingredient, recipe::recipe_ingredient::CompactRecipeIngredient, unit::Unit,
};

/// Generates a random number of `CompactRecipeIngredient` instances from the given units and ingredients.
///
/// This function generates a random number of `CompactRecipeIngredient` instances. Each ingredient is paired
/// with a random unit, and all ingredients are guaranteed to be unique within the generated list.
///
/// # Parameters
/// - `units`: A vector of `Unit` instances to choose from.
/// - `ingredients`: A vector of `Ingredient` instances to choose from.
///
/// # Returns
/// - `Vec<CompactRecipeIngredient>`: A vector containing the generated `CompactRecipeIngredient` instances.
///
/// # Panics
/// This function may panic if:
/// - The `ingredient_id` or `unit_id` of any ingredient or unit cannot be unwrapped.
/// - The conversion from `usize` to `i32` fails.
pub fn generate_random_recipe_ingredients(
    units: Vec<Unit>,
    ingredients: Vec<Ingredient>,
) -> Vec<CompactRecipeIngredient> {
    let number_of_pairs: i32 = (1..=ingredients.len().try_into().unwrap()).fake::<i32>();
    let mut ingredient_ids: HashSet<i32> = HashSet::new(); // Ingredients must be unique!
    let mut recipe_ingredients: Vec<CompactRecipeIngredient> = Vec::new();

    while TryInto::<i32>::try_into(recipe_ingredients.len()).unwrap() != number_of_pairs {
        let random_index = (0..ingredients.len().try_into().unwrap()).fake::<usize>();
        let ingr_id = ingredients[random_index]
            .ingredient_id
            .expect("ingredient should have been able to be unwrapped");
        if ingredient_ids.insert(ingr_id) {
            let random_unit_index = (0..units.len().try_into().unwrap()).fake::<usize>();
            let recipe_ingredient = CompactRecipeIngredient::new(
                0,
                units[random_unit_index].unit_id.unwrap(),
                ingr_id,
                Faker.fake::<String>(),
            );
            recipe_ingredients.push(recipe_ingredient)
        }
    }
    recipe_ingredients
}

/// Chooses a random recipe ID from the database.
///
/// This function queries the database to fetch all recipe IDs and selects one at random.
/// If no recipes are found, it will panic with the message "No recipes were found.".
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `i32`: The randomly chosen recipe ID.
///
/// # Panics
/// This function will panic if:
/// - The query to fetch recipes from the database fails.
/// - No recipes are found in the database.
pub async fn choose_random_recipe_id(pool: &PgPool) -> i32 {
    let recipes = sqlx::query!("SELECT recipe_id from recipe")
        .fetch_all(pool)
        .await
        .expect("No recipes were found.");
    let random_index: usize = (0..recipes.len()).fake::<usize>();
    recipes[random_index].recipe_id
}
