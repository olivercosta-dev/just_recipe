#![allow(dead_code)]

use std::collections::HashSet;

use axum::{body::Body, http::Request};
use fake::{Fake, Faker};
use just_recipe::routes::{Ingredient, RecipeIngredient, RecipeStep, Unit};
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
            _step_id: 0,
            recipe_id: 0,
            step_number,
            instruction: Faker.fake::<String>(),
        })
        .collect()
}

// Returns the persisted recipe_id
pub async fn assert_recipe_persists(pool: &PgPool, recipe_name: &str, description: &str) -> i32 {
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
            recipe_record.description.unwrap().as_str()
        ),
        (recipe_name, description)
    );
    recipe_record.recipe_id
}

pub async fn assert_recipe_ingredients_persist(
    pool: &PgPool,
    recipe_ingredients: Vec<Value>,
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
        let ingredient_id = i32::try_from(ingredient["ingredient_id"].as_i64().unwrap()).unwrap();
        let unit_id = i32::try_from(ingredient["unit_id"].as_i64().unwrap()).unwrap();
        let quantity = ingredient["quantity"].as_str().unwrap();

        let record = records
            .iter()
            .find(|&rec| rec.ingredient_id == ingredient_id)
            .expect("Ingredient record not found");
        assert_eq!(
            (
                record.recipe_id,
                record.ingredient_id,
                record.unit_id.unwrap(),
                record.quantity.as_deref().unwrap()
            ),
            (recipe_id, ingredient_id, unit_id, quantity)
        );
    }
}

pub async fn assert_recipe_steps_persist(pool: &PgPool, recipe_steps: Vec<Value>, recipe_id: i32) {
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
    .expect("Should have gotten a result for the recipe steps.");
    assert_eq!(ordered_recipe_step_records.len(), recipe_steps.len());

    for (index, step) in recipe_steps.iter().enumerate() {
        let step_number = i32::try_from(
            step["step_number"]
                .as_i64()
                .expect("Should have been an integer"),
        )
        .expect("Should have been an i32");
        let instruction = String::from(
            step["instruction"]
                .as_str()
                .expect("Should have been a string"),
        );
        let recipe_step_record = &ordered_recipe_step_records[index];
        let (record_recipe_id, record_step_number, record_instruction) = (
            recipe_step_record.recipe_id.unwrap(),
            recipe_step_record.step_number.unwrap(),
            recipe_step_record.instruction.clone().unwrap(),
        );
        assert_eq!(
            (record_recipe_id, record_step_number, record_instruction),
            (recipe_id, step_number, instruction)
        );
    }
}

pub fn generate_random_recipe_ingredients(
    units: Vec<Unit>,
    ingredients: Vec<Ingredient>,
) -> Vec<RecipeIngredient> {
    let number_of_pairs: i32 = (0..ingredients.len().try_into().unwrap()).fake::<i32>();
    let mut ingredient_ids: HashSet<i32> = HashSet::new(); // Ingredients must be unique!
    let mut recipe_ingredients: Vec<RecipeIngredient> = Vec::new();

    while TryInto::<i32>::try_into(recipe_ingredients.len()).unwrap() != number_of_pairs {
        let random_index = (0..ingredients.len().try_into().unwrap()).fake::<usize>();
        let ingr_id = ingredients[random_index].ingredient_id;
        if ingredient_ids.insert(ingr_id) {
            let random_unit_index = (0..units.len().try_into().unwrap()).fake::<usize>();
            let recipe_ingredient = RecipeIngredient {
                _recipe_id: 0,
                ingredient_id: ingr_id,
                unit_id: units[random_unit_index].unit_id,
                quantity: Faker.fake::<String>(),
            };
            recipe_ingredients.push(recipe_ingredient)
        }
    }
    recipe_ingredients
}

pub fn create_recipe_ingredients_json(recipe_ingredients: &[RecipeIngredient]) -> Vec<Value> {
    recipe_ingredients
        .iter()
        .map(|rec_ingr| {
            json!({
                "ingredient_id": rec_ingr.ingredient_id,
                "unit_id": rec_ingr.unit_id,
                "quantity": rec_ingr.quantity
            })
        })
        .collect()
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
