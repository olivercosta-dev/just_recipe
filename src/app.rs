use axum::{routing::get, Router};

use crate::routes::health_check;

pub fn new_app() -> Router {
    Router::new().route("/", get(health_check))
}