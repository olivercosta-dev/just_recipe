use axum::{routing::get, routing::post, Router};
use sqlx::PgPool;
use crate::routes::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool
}

pub async fn new_app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/units", post(units))
        .with_state(app_state)
}
