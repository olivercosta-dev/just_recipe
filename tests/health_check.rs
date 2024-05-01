mod utils;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use just_recipe::app::{new_app, AppState};
use sqlx::PgPool;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_returns_200_ok() {
    let pool = PgPool::connect("postgres://postgres@localhost/just_recipe")
        .await
        .expect("should have connected to database");
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
