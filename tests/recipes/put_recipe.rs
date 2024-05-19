use std::default;

use axum::http::StatusCode;
use fake::{Fake, Faker};
use just_recipe::{
    application::{app::App, state::AppState},
    ingredient::Ingredient,
    unit::Unit,
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use crate::{
    assert_compact_recipe_ingredients_exist, assert_recipe_exists, assert_recipe_steps_exist,
    choose_random_recipe_id, create_put_request_to,
    create_recipe_steps_json_for_request, fetch_ingredients_and_units,
    generate_random_number_of_steps, generate_random_recipe_ingredients,
};
#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients")
))]
async fn updating_existing_recipe_gets_updated_returns_204_no_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let recipe_id = choose_random_recipe_id(&app_state.pool).await;
    let recipe_name = Faker.fake::<String>();
    let recipe_description = Faker.fake::<String>();
    let (ingredients, units) = fetch_ingredients_and_units(&app_state.pool).await;
    let recipe_ingredients = generate_random_recipe_ingredients(units, ingredients);
    let recipe_steps = generate_random_number_of_steps();
    let json = json!({
        "recipe_id": recipe_id,
        "name": recipe_name,
        "description": recipe_description,
        "ingredients": recipe_ingredients,
        "steps": recipe_steps
    });
    let request = create_put_request_to("recipes", recipe_id, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_recipe_exists(&app_state.pool, &recipe_name, &recipe_description).await;
    assert_recipe_steps_exist(&app_state.pool, &recipe_steps, recipe_id)
        .await
        .unwrap();
    assert_compact_recipe_ingredients_exist(&app_state.pool, &recipe_ingredients, recipe_id).await;

    Ok(())
}
#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients")
))]
async fn updating_non_existing_recipe_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let recipe_id = -1;
    let recipe_name = Faker.fake::<String>();
    let recipe_description = Faker.fake::<String>();
    let (ingredients, units) = fetch_ingredients_and_units(&app_state.pool).await;
    let recipe_ingredients = generate_random_recipe_ingredients(units, ingredients);
    let recipe_steps = create_recipe_steps_json_for_request(generate_random_number_of_steps());
    let json = json!({
        "recipe_id": recipe_id,
        "name": recipe_name,
        "description": recipe_description,
        "ingredients": recipe_ingredients,
        "steps":recipe_steps
    });
    let request = create_put_request_to("recipes", recipe_id, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}
#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients")
))]
async fn updating_recipe_with_non_existent_unit_returns_422_unproccessable_entity(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let recipe_id = 1;
    let recipe_name = Faker.fake::<String>();
    let recipe_description = Faker.fake::<String>();
    let (ingredients, _) = fetch_ingredients_and_units(&app_state.pool).await;
    let units = vec![Unit {
        unit_id: Some(100_000),
        singular_name: Faker.fake::<String>(),
        plural_name: Faker.fake::<String>(),
    }];
    let recipe_ingredients = generate_random_recipe_ingredients(units, ingredients);
    let recipe_steps = create_recipe_steps_json_for_request(generate_random_number_of_steps());
    let json = json!({
        "recipe_id": recipe_id,
        "name": recipe_name,
        "description": recipe_description,
        "ingredients": recipe_ingredients,
        "steps":recipe_steps
    });
    let request = create_put_request_to("recipes", recipe_id, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}
#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients")
))]

async fn updating_recipe_with_non_existent_ingredient_id_returns_422_unproccessable_entity(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let recipe_id = 1;
    let recipe_name = Faker.fake::<String>();
    let recipe_description = Faker.fake::<String>();
    let (_, units) = fetch_ingredients_and_units(&app_state.pool).await;
    let ingredients = vec![Ingredient {
        ingredient_id: Some(100_000),
        singular_name: Faker.fake::<String>(),
        plural_name: Faker.fake::<String>(),
    }];
    let recipe_ing = generate_random_recipe_ingredients(units, ingredients);
    let recipe_ingredients = create_recipe_ingredients_json(&recipe_ing);
    let recipe_steps = generate_random_number_of_steps();
    let json = json!({
        "recipe_id": recipe_id,
        "name": recipe_name,
        "description": recipe_description,
        "ingredients": recipe_ingredients,
        "steps":recipe_steps
    });
    let request = create_put_request_to("recipes", recipe_id, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}
