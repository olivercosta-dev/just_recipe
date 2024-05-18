use std::default;

use axum::{body::to_bytes, http::StatusCode};
use just_recipe::{application::{app::App, state::AppState}, unit::Unit};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use crate::{choose_random_unit, create_get_request_to};
#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn getting_existing_unit_returns_ingredient_and_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let unit = choose_random_unit(&app_state.pool).await;
    let json = json!({}); // this is not needed for a get
    let request = create_get_request_to("units", Some(unit.unit_id.unwrap()),None, json);
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body bytes");

    let response_unit: Unit = serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

    assert_eq!(response_unit.unit_id, unit.unit_id);
    assert_eq!(response_unit.singular_name, unit.singular_name);
    assert_eq!(response_unit.plural_name, unit.plural_name);
    Ok(())
}
#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn getting_non_existent_unit_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let json = json!({}); // won't be needing this
    let request = create_get_request_to("units", Some(-1),None, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
