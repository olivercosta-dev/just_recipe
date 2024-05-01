use axum::{routing::get, routing::post, Router};
use sqlx::PgPool;
use tower_http::catch_panic::CatchPanicLayer;
use crate::routes::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool
}
pub async fn new_app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/units", post(units))
        .route("/ingredients", post(ingredients))
        .route("/recipes", post(recipes))
        .layer(CatchPanicLayer::new())
        .with_state(app_state)
}
