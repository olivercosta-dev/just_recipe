use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use just_recipe::app::new_app;
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn health_check_returns_200_ok() {
    let app = new_app();
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
