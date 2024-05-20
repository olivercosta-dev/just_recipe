use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    application::{error::AppError, state::AppState},
    unit::{
        helpers::{delete_unit, insert_unit, update_unit},
        Unit,
    },
    utilities::{
        fetchers::{fetch_unit, fetch_units_with_pagination},
        queries::PaginationQuery,
    },
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUnitRequest {
    pub unit_id: i32,
}
pub async fn add_unit_handler(
    State(app_state): State<AppState>,
    Json(unit): Json<Unit>,
) -> Result<StatusCode, AppError> {
    let unit_id = insert_unit(&unit, &app_state.pool).await?;
    cache_unit_id(unit_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}

fn cache_unit_id(unit_id: i32, app_state: AppState) {
    app_state.unit_ids.insert(unit_id);
}

pub async fn remove_unit_handler(
    State(app_state): State<AppState>,
    Json(delete_unit_request): Json<DeleteUnitRequest>,
) -> Result<StatusCode, AppError> {
    delete_unit(&delete_unit_request.unit_id, &app_state.pool).await?;
    remove_unit_id_from_cache(&delete_unit_request.unit_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}

fn remove_unit_id_from_cache(unit_id: &i32, app_state: AppState) {
    app_state.unit_ids.remove(unit_id);
}

pub async fn update_unit_handler(
    State(app_state): State<AppState>,
    Path(unit_id): Path<i32>,
    Json(unit): Json<Unit>,
) -> Result<StatusCode, AppError> {
    update_unit(&app_state.pool, unit_id, unit).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_unit_handler(
    State(app_state): State<AppState>,
    Path(unit_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let unit = fetch_unit(&app_state.pool, unit_id).await?;
    Ok(Json(unit))
}

#[derive(Serialize, Deserialize)]
pub struct GetUnitsResponse {
    pub units: Vec<Unit>,
    // The id from which the next batch is accesbile.
    // The ID and details themselves will be contained in the next response, not this one.
    // It is none if there are no more units for the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_start_from: Option<i32>,
}

pub async fn get_units_by_query_handler(
    State(app_state): State<AppState>,
    query: Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    if query.limit > 15 || query.limit < 1 {
        return Err(AppError::BadRequest);
    }
    let mut units: Vec<Unit> = fetch_units_with_pagination(&query, &app_state.pool).await?;
    let next_start_from: Option<i32> = {
        // We are casting length upwards so that it is not lossy.
        // It is (<=) because the vector we have in ingredients
        // is always going to try to fetch 1 more ingredient.
        if (units.len() as i64) <= query.limit {
            None
        } else {
            units.pop().and_then(|unit| unit.unit_id)
        }
    };
    let response = GetUnitsResponse {
        units,
        next_start_from,
    };
    Ok(Json(response))
}
