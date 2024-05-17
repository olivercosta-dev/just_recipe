use std::default;

use axum::{body::to_bytes, http::StatusCode};
use fake::{Fake, Faker};
use just_recipe::{application::{app::App, state::AppState}, ingredient::Ingredient, unit::Unit};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use crate::{choose_random_ingredient, create_get_request_to};
#[sqlx::test(fixtures(path = "../fixtures", scripts("ingredients")))]
async fn getting_existing_ingredient_returns_ingredient_and_200_ok(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let ingredient = choose_random_ingredient(&app_state.pool).await;
    let json = json!({}); // this is not needed for a get
    let request = create_get_request_to("ingredients", ingredient.ingredient_id.unwrap(), json);
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body bytes");

    let response_ingredient: Ingredient =
        serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

    assert_eq!(response_ingredient.ingredient_id, ingredient.ingredient_id);
    assert_eq!(response_ingredient.singular_name, ingredient.singular_name);
    assert_eq!(response_ingredient.plural_name, ingredient.plural_name);
    Ok(())
}
#[sqlx::test(fixtures(path = "../fixtures", scripts("ingredients")))]
async fn getting_non_existent_ingredient_returns_404_not_found(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let json = json!({}); // won't be needing this
    let request = create_get_request_to("ingredients", -1, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
