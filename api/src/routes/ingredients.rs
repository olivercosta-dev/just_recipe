use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    application::{error::AppError, state::AppState},
    ingredient::{
        helpers::{delete_ingredient, insert_ingredient, update_ingredient},
        Ingredient,
    },
    utilities::{
        fetchers::{fetch_ingredient, fetch_ingredients_with_pagination},
        queries::PaginationQuery,
    },
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveIngredientRequest {
    pub ingredient_id: i32,
}
// TODO (oliver): Return the ingredient_id in the JSON response!
pub async fn add_ingredient_handler(
    State(app_state): State<AppState>,
    Json(ingredient): Json<Ingredient>,
) -> Result<StatusCode, AppError> {
    let ingredient_id = insert_ingredient(&ingredient, &app_state.pool).await?;
    cache_ingredient_id(ingredient_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}
fn cache_ingredient_id(ingredient_id: i32, app_state: AppState) {
    app_state.ingredient_ids.insert(ingredient_id);
}
// TODO (oliver): This should use Path instead of a json!
pub async fn remove_ingredient_handler(
    State(app_state): State<AppState>,
    Json(delete_ingredient_request): Json<RemoveIngredientRequest>,
) -> Result<StatusCode, AppError> {
    delete_ingredient(&delete_ingredient_request.ingredient_id, &app_state.pool).await?;
    remove_ingredient_id_from_cache(&delete_ingredient_request.ingredient_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}

fn remove_ingredient_id_from_cache(ingredient_id: &i32, app_state: AppState) {
    app_state.ingredient_ids.remove(ingredient_id);
}
pub async fn update_ingredient_handler(
    State(app_state): State<AppState>,
    Path(ingredient_id): Path<i32>,
    Json(ingredient): Json<Ingredient>,
) -> Result<StatusCode, AppError> {
    update_ingredient(ingredient, ingredient_id, &app_state.pool).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_ingredient_by_id_handler(
    State(app_state): State<AppState>,
    Path(ingredient_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let ingredient = fetch_ingredient(&app_state.pool, ingredient_id).await?;
    Ok(Json(ingredient))
}

#[derive(Serialize, Deserialize)]
pub struct GetIngredientsResponse {
    pub ingredients: Vec<Ingredient>,
    // The id from which the next batch is accesbile.
    // This id will be contained in the next response, but not this one.
    // It is none if there are no more ingredients for the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_start_from: Option<i32>,
}

pub async fn get_ingredients_by_query_handler(
    State(app_state): State<AppState>,
    query: Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    if query.limit > 15 || query.limit < 1 {
        return Err(AppError::BadRequest);
    }
    let mut ingredients: Vec<Ingredient> =
        fetch_ingredients_with_pagination(&query, &app_state.pool).await?;
    let next_start_from: Option<i32> = {
        // We are casting length upwards so that it is not lossy.
        // It is (<=) because the vector we have in ingredients
        // is always going to try to fetch 1 more ingredient.
        if (ingredients.len() as i64) <= query.limit {
            None
        } else {
            ingredients.pop().and_then(|ingr| ingr.ingredient_id)
        }
    };
    let response = GetIngredientsResponse {
        ingredients,
        next_start_from,
    };
    
    Ok(Json(response))
}
