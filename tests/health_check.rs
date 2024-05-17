mod utils;

use std::default;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use just_recipe::application::{app::App, state::AppState};
use sqlx::PgPool;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_returns_200_ok() {
    let pool = PgPool::connect("postgres://postgres@localhost/just_recipe")
        .await
        .expect("should have connected to database");
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let response = app
        .router
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
