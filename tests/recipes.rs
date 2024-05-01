// TODO (oliver): Create a utils crate for tests

use std::collections::HashSet;

use axum::http::StatusCode;
use fake::{Fake, Faker};
use just_recipe::{
    app::{new_app, AppState},
    routes::{Ingredient, RecipeIngredient, RecipeStep, Unit},
    utils::create_post_request_to,
};

use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

#[sqlx::test(fixtures("units", "ingredients"))]
async fn adding_new_recipe_persists_and_returns_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;

    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();

    let recipe_steps = create_recipe_steps_json_for_request(generate_random_number_of_steps());

    let (all_ingredients, all_units) = fetch_ingredients_and_units(&app_state.pool).await;
    let recipe_ingredients: Vec<Value> = create_recipe_ingredients_json(
        &generate_random_recipe_ingredients(all_units, all_ingredients),
    );

    let json = json!(
        {
            "name": recipe_name,
            "description": description,
            "ingredients": recipe_ingredients,
            "steps": recipe_steps
        }
    );
    let request = create_post_request_to("recipes", json);
    let response = app
        .oneshot(request)
        .await
        .expect("Should have gotten a valid response.");

    assert_eq!(response.status(), StatusCode::OK,);

    let recipe_id = assert_recipe_persists(&app_state.pool, &recipe_name, &description).await;

    assert_recipe_ingredients_persist(&app_state.pool, recipe_ingredients, recipe_id).await;
    assert_recipe_steps_persist(&app_state.pool, recipe_steps, recipe_id).await;

    Ok(())
}

fn create_recipe_steps_json_for_request(steps: Vec<RecipeStep>) -> Vec<Value> {
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

fn generate_random_number_of_steps() -> Vec<RecipeStep> {
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
async fn assert_recipe_persists(pool: &PgPool, recipe_name: &str, description: &str) -> i32 {
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

async fn assert_recipe_ingredients_persist(
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

async fn assert_recipe_steps_persist(pool: &PgPool, recipe_steps: Vec<Value>, recipe_id: i32) {
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
fn generate_random_recipe_ingredients(
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
            let recipe_ingredient = RecipeIngredient {
                _recipe_id: 0,
                ingredient_id: ingr_id,
                unit_id: (0..units.len().try_into().unwrap()).fake::<i32>(),
                quantity: Faker.fake::<String>(),
            };
            recipe_ingredients.push(recipe_ingredient)
        }
    }
    recipe_ingredients
}

fn create_recipe_ingredients_json(recipe_ingredients: &[RecipeIngredient]) -> Vec<Value> {
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

async fn fetch_ingredients_and_units(pool: &PgPool) -> (Vec<Ingredient>, Vec<Unit>) {
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

#[sqlx::test(fixtures("units", "ingredients"))]
async fn adding_recipe_with_wrong_step_numbers_returns_422_unproccessable_entity(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let (all_ingredients, all_units) = fetch_ingredients_and_units(&app_state.pool).await;
    let ingredients = create_recipe_ingredients_json(&generate_random_recipe_ingredients(
        all_units,
        all_ingredients,
    ));
    let (step_number1, instruction1) = (1, Faker.fake::<String>());
    let (wrong_step_number, instruction2) = (7, Faker.fake::<String>());

    let json = json!(
        {
            "name": recipe_name,
            "description": description,
            "ingredients": ingredients,
            "steps": [
                {
                    "step_number": step_number1,
                    "instruction": instruction1
                },
                {
                    "step_number": wrong_step_number,
                    "instruction": instruction2
                }
            ]
        }
    );

    let request = create_post_request_to("recipes", json);
    let response = app
        .oneshot(request)
        .await
        .expect("Should have gotten a valid response.");
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[sqlx::test(fixtures("units"))]
async fn adding_recipe_with_non_existent_ingredient_id_returns_422_unproccessable_entity(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let (ingredient_id, unit_id, quantity) = (Faker.fake::<i32>(), 1, String::from("3/4"));
    let steps: Vec<Value> = create_recipe_steps_json_for_request(generate_random_number_of_steps());
    let json = json!(
        {
            "name": recipe_name,
            "description": description,
            "ingredients": [
                {
                    "ingredient_id": ingredient_id,
                    "unit_id": unit_id,
                    "quantity": quantity,
                }
            ],
            "steps": steps
        }
    );
    let request = create_post_request_to("recipes", json);
    let response = app
        .oneshot(request)
        .await
        .expect("Should have gotten a valid response.");
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}

#[sqlx::test(fixtures("ingredients"))]
async fn adding_recipe_with_non_existent_unit_id_returns_422_unproccessable_entity(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let (ingredient_id, unit_id, quantity) = (1, Faker.fake::<i32>(), String::from("3/4"));
    let steps: Vec<Value> = create_recipe_steps_json_for_request(generate_random_number_of_steps());
    let json = json!(
        {
            "name": recipe_name,
            "description": description,
            "ingredients": [
                {
                    "ingredient_id": ingredient_id,
                    "unit_id": unit_id,
                    "quantity": quantity,
                }
            ],
            "steps": steps
        }
    );
    let request = create_post_request_to("recipes", json);
    let response = app
        .oneshot(request)
        .await
        .expect("Should have gotten a valid response.");
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}

#[sqlx::test(fixtures("ingredients", "units"))]
async fn adding_recipe_with_duplicate_ingredient_ids_returns_422_unproccessable_entity(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let (ingredient_id1, unit_id1, quantity1) = (1, 1, String::from("3/4")); // Notice ingredient_id1 and ingredient_id2 are the same.
    let (ingredient_id2, unit_id2, quantity2) = (1, 1, String::from("1/4"));
    let steps = create_recipe_steps_json_for_request(generate_random_number_of_steps());
    let json = json!(
        {
            "name": recipe_name,
            "description": description,
            "ingredients": [
                {
                    "ingredient_id": ingredient_id1,
                    "unit_id": unit_id1,
                    "quantity": quantity1,
                },
                {
                    "ingredient_id": ingredient_id2,
                    "unit_id": unit_id2,
                    "quantity": quantity2,
                }
            ],
            "steps":steps
        }
    );
    let request = create_post_request_to("recipes", json);
    let response = app
        .oneshot(request)
        .await
        .expect("Should have gotten a valid response.");
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}
