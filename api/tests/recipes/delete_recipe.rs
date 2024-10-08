use std::default;

use axum::http::StatusCode;
use just_recipe::{
    application::{app::App, state::AppState},
    utilities::{
        random_generation::recipes::choose_random_recipe_id,
        request_creators::create_delete_request_to,
    },
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients", "steps")
))]
async fn deleting_existing_recipe_gets_removed_returns_204_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let choose_random_recipe_id = choose_random_recipe_id(&app_state.pool);
    let recipe_id = choose_random_recipe_id.await;
    let request = create_delete_request_to("recipes", json!({"recipe_id": recipe_id}));
    let response = app.router.oneshot(request).await.unwrap();

    let recipe_record = sqlx::query!(
        "SELECT recipe_id FROM recipe WHERE recipe_id = $1",
        recipe_id
    )
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();
    let recipe_steps_records =
        sqlx::query!("SELECT recipe_id FROM step WHERE recipe_id = $1", recipe_id)
            .fetch_optional(&app_state.pool)
            .await
            .unwrap();
    let recipe_ingredients_records = sqlx::query!(
        "SELECT recipe_id FROM recipe_ingredient WHERE recipe_id = $1",
        recipe_id
    )
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert!(recipe_record.is_none());
    assert!(recipe_ingredients_records.is_none());
    assert!(recipe_steps_records.is_none());
    Ok(())
}
#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients")
))]
async fn deleting_non_existent_recipe_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let recipe_id = -1;
    let request = create_delete_request_to("recipes", json!({"recipe_id": recipe_id}));
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
