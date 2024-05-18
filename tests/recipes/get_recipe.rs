use std::default;

use axum::{body::to_bytes, http::StatusCode};
use itertools::Itertools;
use just_recipe::{
    application::{app::App, state::AppState},
    recipe::{
        recipe::Recipe,
        recipe_ingredient::{DetailedRecipeIngredient, RecipeIngredient},
    }
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use crate::{
    assert_detailed_recipe_ingredients_exist, assert_recipe_exists, assert_recipe_steps_exist,
    choose_random_recipe_id, create_get_request_to,
};

#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients", "steps")
))]
async fn getting_existing_recipe_returns_recipe_and_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let recipe_id = choose_random_recipe_id(&app_state.pool).await;
    let request = create_get_request_to("recipes", Some(recipe_id), None, json!({}));
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body bytes");

    let response_recipe: Recipe<DetailedRecipeIngredient> =
        serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

    let recipe_id_in_db = assert_recipe_exists(
        &app_state.pool,
        response_recipe.name(),
        response_recipe.description(),
    )
    .await;
    assert_eq!(
        response_recipe
            .recipe_id()
            .expect("recipe_id should have existed"),
        &recipe_id_in_db
    );

    assert_recipe_steps_exist(
        &app_state.pool,
        response_recipe
            .steps()
            .iter()
            .map(|f| json!(f))
            .collect_vec(),
        recipe_id,
    )
    .await;
    assert_detailed_recipe_ingredients_exist(
        &app_state.pool,
        response_recipe
            .ingredients()
            .iter()
            .map(|ing| ing.ingredient())
            .collect(),
    )
    .await;
    Ok(())
}

#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients", "steps")
))]
async fn getting_non_existent_recipe_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let recipe_id = -1;
    let request = create_get_request_to("recipes", Some(recipe_id), None, json!({}));
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
