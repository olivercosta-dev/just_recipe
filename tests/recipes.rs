use axum::http::StatusCode;
use fake::{Fake, Faker};
use just_recipe::app::{new_app, AppState};

mod utils;
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;
use utils::*;

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
