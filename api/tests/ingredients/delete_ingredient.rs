use std::default;

use axum::http::StatusCode;
use just_recipe::{
    application::{app::App, state::AppState},
    utilities::{
        random_generation::ingredients::choose_random_ingredient,
        request_creators::create_delete_request_to,
    },
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test(fixtures(path = "../fixtures", scripts("ingredients")))]
async fn deleting_non_existent_ingredient_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let ingredient_id = -1;
    let request = create_delete_request_to("ingredients", json!({"ingredient_id": ingredient_id}));
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
#[sqlx::test(fixtures(path = "../fixtures", scripts("ingredients")))]
async fn deleting_existing_ingredient_gets_removed_returns_204_no_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let choose_random_ingredient = choose_random_ingredient(&app_state.pool);
    let ingredient_id = choose_random_ingredient.await.ingredient_id;
    let request = create_delete_request_to("ingredients", json!({"ingredient_id": ingredient_id}));
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let ingredient_record = sqlx::query!(
        "SELECT ingredient_id from ingredient where ingredient_id = $1",
        ingredient_id
    )
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();
    assert!(ingredient_record.is_none());
    Ok(())
}
