use std::default;

use axum::http::StatusCode;
use just_recipe::{
    application::{app::App, state::AppState},
    utilities::{
        random_generation::units::choose_random_unit, request_creators::create_delete_request_to,
    },
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn deleting_existing_unit_gets_removed_returns_204_no_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let unit_id = choose_random_unit(&app_state.pool).await.unit_id;
    let request = create_delete_request_to("units", json!({"unit_id": unit_id}));
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let unit_record = sqlx::query!("SELECT unit_id from unit where unit_id = $1", unit_id)
        .fetch_optional(&app_state.pool)
        .await
        .unwrap();
    assert!(unit_record.is_none());
    Ok(())
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn deleting_non_existent_unit_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let unit_id = -1;
    let request = create_delete_request_to("units", json!({"unit_id": unit_id}));
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
