use crate::{fetch_all_ingredient_ids, fetch_all_unit_ids, recipe::RecipeParsingError, routes::*};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Router,
};
use dashmap::DashSet;
use sqlx::{Error as SqlxError, PgPool};
use tower_http::catch_panic::CatchPanicLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub unit_ids: DashSet<i32>,
    pub ingredient_ids: DashSet<i32>,
}
#[derive(Debug, PartialEq)]
pub enum AppError {
    InternalServerError,
    NotFound,
    Conflict,
    RecipeParsingError(RecipeParsingError),
}

impl AppState {
    /// ## This function might panic.
    pub async fn new(pool: PgPool) -> AppState {
        let unit_ids = fetch_all_unit_ids(&pool)
            .await
            .expect("should have fetched the unit_id set");
        let ingredient_ids = fetch_all_ingredient_ids(&pool)
            .await
            .expect("should have fetched the ingredient_id set");
        AppState {
            pool,
            unit_ids,
            ingredient_ids,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::RecipeParsingError(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
        .into_response()
    }
}

impl From<SqlxError> for AppError {
    fn from(_: SqlxError) -> Self {
        AppError::InternalServerError
    }
}

pub async fn new_app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/units", post(add_unit).delete(remove_unit))
        .route("/units/:unit_id", put(update_unit).get(get_unit))
        .route(
            "/ingredients",
            post(add_ingredient).delete(remove_ingredient),
        )
        .route(
            "/ingredients/:ingredient_id",
            put(update_ingredient).get(get_ingredient),
        )
        .route("/recipes", post(add_recipe).delete(remove_recipe))
        .route("/recipes/:recipe_id", put(update_recipe))
        .layer(CatchPanicLayer::new())
        .with_state(app_state)
}
