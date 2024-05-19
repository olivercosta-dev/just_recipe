pub mod ingredients;
pub mod recipes;
pub mod units;

use std::collections::HashSet;

use axum::{body::Body, http::Request};
use fake::{Fake, Faker};
use itertools::Itertools;
use just_recipe::{
    ingredient::Ingredient,
    recipe::{
        recipe_ingredient::{CompactRecipeIngredient, DetailedRecipeIngredient, RecipeIngredient},
        recipe_step::RecipeStep,
    },
    unit::Unit,
};
use serde_json::{json, Value};
use sqlx::PgPool;

pub fn create_post_request_to(endpoint: &str, json: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(format!("/{}", endpoint))
        .header("Content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json).unwrap()))
        .unwrap()
}

pub fn create_delete_request_to(endpoint: &str, json: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("DELETE")
        .uri(format!("/{}", endpoint))
        .header("Content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json).unwrap()))
        .unwrap()
}

pub fn create_put_request_to(
    endpoint: &str,
    resource_id: i32,
    json: serde_json::Value,
) -> Request<Body> {
    Request::builder()
        .method("PUT")
        .uri(format!("/{}/{}", endpoint, resource_id))
        .header("Content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json).unwrap()))
        .unwrap()
}

pub fn create_get_request_to(
    endpoint: &str,
    resource_id: Option<i32>,
    query_params: Option<String>,
    json: serde_json::Value,
) -> Request<Body> {
    let resource;

    if resource_id.is_none() {
        resource = String::from("");
    } else {
        resource = format!("/{}", resource_id.unwrap());
    }
    let query;
    if query_params.is_some() {
        query = "?".to_owned() + &query_params.unwrap();
    } else {
        query = String::from("");
    }
    Request::builder()
        .method("GET")
        .uri(format!("/{}{}{}", endpoint, resource, query))
        .body(Body::from(serde_json::to_vec(&json).unwrap()))
        .unwrap()
}

pub fn create_recipe_steps_json_for_request(steps: Vec<RecipeStep>) -> Vec<Value> {
    steps
        .iter()
        .map(|step| {
            json!({
                "step_number": step.step_number,
                "instruction": step.instruction
            })
        })
        .collect()
}

pub fn generate_random_number_of_steps() -> Vec<RecipeStep> {
    let number_of_steps = (2..10).fake::<i32>();
    (1..number_of_steps)
        .map(|step_number| RecipeStep {
            step_id: 0,
            recipe_id: 0,
            step_number,
            instruction: Faker.fake::<String>(),
        })
        .collect()
}

// Returns the persisted recipe_id
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


/// Checks if all recipe steps exist in the database.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool.
/// * `recipe_steps` - A vector of `RecipeStep` objects.
/// * `recipe_id` - The ID of the recipe.
///
/// # Errors
///
/// This function will return an error if any database operation fails or if the number of
/// steps in the database does not match the number of provided steps, or if any step's details
/// do not match.
pub async fn assert_recipe_steps_exist(
    pool: &PgPool,
    recipe_steps: &[RecipeStep],
    recipe_id: i32
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
    .await.unwrap();

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

        if (record_recipe_id, record_step_number, record_instruction.clone()) != (recipe_id, step.step_number, step.instruction.clone()) {
            return Err(format!(
                "Step mismatch at index {}: expected ({}, {}, {}), found ({}, {}, {})",
                index,
                recipe_id, step.step_number, step.instruction,
                record_recipe_id, record_step_number, record_instruction
            ));
        }
    }

    Ok(())
}
// TODO (oliver): This code isn't fully safe yet. It can panic with some length discrepancies
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


pub async fn fetch_ingredients_and_units(pool: &PgPool) -> (Vec<Ingredient>, Vec<Unit>) {
    let all_ingredients = sqlx::query_as!(Ingredient, "SELECT * FROM ingredient")
        .fetch_all(pool)
        .await
        .expect("Should have had at least 1 ingredient in the database");

    let all_units = sqlx::query_as!(Unit, "SELECT * FROM unit")
        .fetch_all(pool)
        .await
        .expect("Should have had at least 1 unit in the database");

    (all_ingredients, all_units)
}

pub async fn choose_random_ingredient(pool: &PgPool) -> Ingredient {
    let ingredients = sqlx::query_as!(Ingredient, "SELECT * from ingredient")
        .fetch_all(pool)
        .await
        .expect("No ingredients were found.");
    let random_index = (0..ingredients.len()).fake::<usize>();
    Ingredient {
        ingredient_id: ingredients[random_index].ingredient_id,
        singular_name: ingredients[random_index].singular_name.clone(),
        plural_name: ingredients[random_index].plural_name.clone(),
    }
}

pub async fn choose_random_unit(pool: &PgPool) -> Unit {
    let units = sqlx::query_as!(Unit, "SELECT * from unit")
        .fetch_all(pool)
        .await
        .expect("No units were found.");
    let random_index = (0..units.len()).fake::<usize>();
    Unit {
        unit_id: units[random_index].unit_id,
        singular_name: units[random_index].singular_name.clone(),
        plural_name: units[random_index].plural_name.clone(),
    }
}

pub async fn choose_random_recipe_id(pool: &PgPool) -> i32 {
    let recipes = sqlx::query!("SELECT recipe_id from recipe")
        .fetch_all(pool)
        .await
        .expect("No recipes were found.");
    let random_index: usize = (0..recipes.len()).fake::<usize>();
    recipes[random_index].recipe_id
}



pub async fn assert_ingredients_exist(
    pool: &PgPool,
    ingredients: Vec<&Ingredient>,
) {
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

/// Checks if all detailed ingredients exist in the database.
///
/// # Arguments
///
/// * `pool` - A reference to the database connection pool.
/// * `ingredients` - A vector of references to `DetailedRecipeIngredient` objects.
///
/// # Errors
///
/// This function will return an error if any database operation fails or if any
/// ingredient does not exist in the database.
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
            .await.unwrap();

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

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use just_recipe::{ingredient::Ingredient, unit::Unit};

    use super::generate_random_recipe_ingredients;

    #[test]
    fn generates_random_recipe_ingredients() {
        let units = vec![Unit {
            unit_id: Some(1),
            singular_name: Faker.fake::<String>(),
            plural_name: Faker.fake::<String>(),
        }];
        let ingredients = vec![Ingredient {
            ingredient_id: Some(1),
            singular_name: Faker.fake::<String>(),
            plural_name: Faker.fake::<String>(),
        }];
        let recipe_ingredients = generate_random_recipe_ingredients(units, ingredients);
        assert!(recipe_ingredients.len() > 0);
    }
}
