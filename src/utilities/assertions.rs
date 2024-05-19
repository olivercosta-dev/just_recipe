use itertools::Itertools;
use sqlx::PgPool;

use crate::{
    ingredient::Ingredient,
    recipe::{
        recipe_ingredient::{CompactRecipeIngredient, DetailedRecipeIngredient, RecipeIngredient},
        recipe_step::RecipeStep,
    },
};

/// Asserts that a recipe with the specified name and description exists in the database.
///
/// This function checks if a recipe with the given name and description exists in the database.
/// If the recipe is found, it asserts that the name and description match the provided values.
/// It then returns the ID of the recipe.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
/// - `recipe_name`: A string slice representing the name of the recipe to check.
/// - `description`: A string slice representing the description of the recipe to check.
///
/// # Returns
/// - `i32`: The ID of the recipe if it exists.
///
/// # Panics
/// This function will panic if:
/// - The query to fetch the recipe from the database fails.
/// - The recipe is not found in the database.
/// - The name or description of the recipe in the database does not match the provided values.
pub async fn assert_recipe_exists(pool: &PgPool, recipe_name: &str, description: &str) -> i32 {
    let recipe_record = sqlx::query!(
        r#"
            SELECT recipe_id, name, description
            FROM recipe
            WHERE name = $1 and description = $2;
        "#,
        recipe_name,
        description
    )
    .fetch_one(pool)
    .await
    .expect("Should have gotten a record of a recipe.");

    assert_eq!(
        (
            recipe_record.name.as_str(),
            recipe_record.description.as_str()
        ),
        (recipe_name, description)
    );
    recipe_record.recipe_id
}

/// Asserts that all provided `CompactRecipeIngredient` instances exist in the database for a given recipe.
///
/// This function checks if the ingredients associated with a specific recipe are present in the
/// database and verifies that their details match. It ensures that the number of ingredients
/// in the database is the same as the number of ingredients provided, and that each ingredient's
/// details (recipe ID, ingredient ID, unit ID, and quantity) match those in the database.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
/// - `recipe_ingredients`: A slice of `CompactRecipeIngredient` instances to verify.
/// - `recipe_id`: The ID of the recipe to which the ingredients belong.
///
/// # Panics
/// This function will panic if:
/// - The query to fetch ingredients from the database fails.
/// - The number of ingredients in the database does not match the number of ingredients provided.
/// - Any ingredient provided does not have a matching record in the database.
/// - The details of any ingredient do not match the corresponding record in the database.
pub async fn assert_compact_recipe_ingredients_exist(
    pool: &PgPool,
    recipe_ingredients: &[CompactRecipeIngredient],
    recipe_id: i32,
) {
    let records = sqlx::query!(
        r#"
            SELECT recipe_id, ingredient_id, unit_id, quantity
            FROM recipe_ingredient
            WHERE recipe_id = $1
            ORDER BY ingredient_id;
        "#,
        recipe_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    assert_eq!(records.len(), recipe_ingredients.len());

    for ingredient in recipe_ingredients {
        let ingredient_id = ingredient.ingredient();
        let unit_id = ingredient.unit();
        let quantity = ingredient.quantity();

        let record = records
            .iter()
            .find(|&rec| rec.ingredient_id == *ingredient_id)
            .expect("Ingredient record not found");
        assert_eq!(
            (
                record.recipe_id,
                record.ingredient_id,
                record.unit_id,
                record.quantity.as_str()
            ),
            (recipe_id, *ingredient_id, *unit_id, quantity)
        );
    }
}

/// Asserts that all provided `DetailedRecipeIngredient` instances exist in the database.
///
/// This function checks if the ingredients associated with a specific recipe are present in the
/// database. It iterates through each ingredient, verifies if its ID exists in the database,
/// and returns an error if any ingredient is not found or if an ingredient ID is missing.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
/// - `ingredients`: A slice of `DetailedRecipeIngredient` instances to verify.
///
/// # Returns
/// - `Result<(), String>`: Returns `Ok(())` if all ingredients exist. Returns an `Err(String)`
///   with a descriptive error message if any ingredient does not exist or has a missing ID.
pub async fn assert_detailed_ingredients_exist(
    pool: &PgPool,
    ingredients: &[DetailedRecipeIngredient],
) -> Result<(), String> {
    for ingr in ingredients.iter() {
        if let Some(ingredient_id) = ingr.ingredient().ingredient_id {
            let result: Option<Ingredient> = sqlx::query_as!(
                Ingredient,
                r#"
                    SELECT *
                    FROM ingredient
                    WHERE ingredient_id = $1;
                "#,
                ingredient_id
            )
            .fetch_optional(pool)
            .await
            .unwrap();

            if result.is_none() {
                return Err(format!(
                    "Ingredient with ID {} does not exist in the database.",
                    ingredient_id
                ));
            }
        } else {
            return Err(format!(
                "Ingredient ID is None for one of the provided ingredients."
            ));
        }
    }

    Ok(())
}

/// Asserts that all provided `Ingredient` instances exist in the database.
///
/// This function checks if the ingredients in the provided vector are present in the
/// database. It verifies that each ingredient's ID exists in the database and asserts
/// that the number of ingredients found matches the number of ingredients provided.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
/// - `ingredients`: A vector of references to `Ingredient` instances to verify.
///
/// # Panics
/// This function will panic if:
/// - The query to fetch ingredients from the database fails.
/// - Any ingredient ID is `None`.
/// - The number of ingredients in the database does not match the number of ingredients provided.
///
pub async fn assert_ingredients_exist(pool: &PgPool, ingredients: Vec<&Ingredient>) {
    let ingr_ids = ingredients
        .iter()
        .map(|ingr| ingr.ingredient_id.unwrap())
        .collect_vec();
    let ingr_records = sqlx::query_as!(
        Ingredient,
        r#"
            SELECT *
            FROM ingredient
            WHERE ingredient_id = ANY($1);
        "#,
        &ingr_ids
    )
    .fetch_all(pool)
    .await
    .unwrap();

    assert_eq!(ingr_records.len(), ingredients.len());
}

/// Asserts that all provided `RecipeStep` instances exist in the database for a given recipe.
///
/// This function checks if the steps associated with a specific recipe are present in the
/// database. It verifies that the number of steps and their details (step number and instruction)
/// match the provided `RecipeStep` instances. If any discrepancy is found, it returns an error
/// with a descriptive message.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
/// - `recipe_steps`: A slice of `RecipeStep` instances to verify.
/// - `recipe_id`: The ID of the recipe to which the steps belong.
///
/// # Returns
/// - `Result<(), String>`: Returns `Ok(())` if all steps exist and match the provided details.
///   Returns an `Err(String)` with a descriptive error message if any discrepancy is found.
pub async fn assert_recipe_steps_exist(
    pool: &PgPool,
    recipe_steps: &[RecipeStep],
    recipe_id: i32,
) -> Result<(), String> {
    let ordered_recipe_step_records = sqlx::query!(
        r#"
            SELECT step_id, recipe_id, step_number, instruction
            FROM step
            WHERE recipe_id = $1
            ORDER BY step_number;
        "#,
        recipe_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    if ordered_recipe_step_records.len() != recipe_steps.len() {
        return Err(format!(
            "Number of steps mismatch. Expected: {}, Found: {}",
            recipe_steps.len(),
            ordered_recipe_step_records.len()
        ));
    }

    for (index, step) in recipe_steps.iter().enumerate() {
        let recipe_step_record = &ordered_recipe_step_records[index];
        let (record_recipe_id, record_step_number, record_instruction) = (
            recipe_step_record.recipe_id,
            recipe_step_record.step_number,
            recipe_step_record.instruction.clone(),
        );

        if (
            record_recipe_id,
            record_step_number,
            record_instruction.clone(),
        ) != (recipe_id, step.step_number, step.instruction.clone())
        {
            return Err(format!(
                "Step mismatch at index {}: expected ({}, {}, {}), found ({}, {}, {})",
                index,
                recipe_id,
                step.step_number,
                step.instruction,
                record_recipe_id,
                record_step_number,
                record_instruction
            ));
        }
    }

    Ok(())
}
