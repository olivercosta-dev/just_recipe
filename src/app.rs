use crate::routes::*;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::catch_panic::CatchPanicLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
pub async fn new_app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/units", post(add_unit).delete(remove_unit))
        .route(
            "/ingredients",
            post(add_ingredient).delete(remove_ingredient),
        )
        .route("/recipes", post(add_recipe).delete(remove_recipe))
        .layer(CatchPanicLayer::new())
        .with_state(app_state)
}
