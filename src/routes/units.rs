use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};

use crate::{
    application::{error::AppError, state::AppState},
    unit::Unit,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUnitRequest {
    pub unit_id: i32,
}
pub async fn add_unit(
    State(app_state): State<AppState>,
    Json(unit): Json<Unit>,
) -> Result<StatusCode, AppError> {
    let unit_id = insert_unit_into_db(&unit, &app_state.pool).await?;
    cache_unit_id(unit_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}

async fn insert_unit_into_db(unit: &Unit, pool: &PgPool) -> Result<i32, AppError> {
    match query!(
        r#"
            INSERT INTO unit (singular_name, plural_name) 
            VALUES ($1, $2)
            RETURNING unit_id;
        "#,
        unit.singular_name,
        unit.plural_name
    )
    .fetch_one(pool)
    .await
    {
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Err(AppError::Conflict),
        Err(_) => Err(AppError::InternalServerError),
        Ok(rec) => Ok(rec.unit_id),
    }
}

fn cache_unit_id(unit_id: i32, app_state: AppState) {
    app_state.unit_ids.insert(unit_id);
}
fn remove_unit_id_from_cache(unit_id: &i32, app_state: AppState) {
    app_state.unit_ids.remove(unit_id);
}

pub async fn remove_unit(
    State(app_state): State<AppState>,
    Json(delete_unit_request): Json<DeleteUnitRequest>,
) -> Result<StatusCode, AppError> {
    delete_unit_from_db(&delete_unit_request.unit_id, &app_state.pool).await?;
    remove_unit_id_from_cache(&delete_unit_request.unit_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}
async fn delete_unit_from_db(unit_id: &i32, pool: &PgPool) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM unit WHERE unit_id = $1", unit_id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

pub async fn update_unit(
    State(app_state): State<AppState>,
    Path(unit_id): Path<i32>,
    Json(unit): Json<Unit>,
) -> Result<StatusCode, AppError> {
    update_unit_record(&app_state.pool, unit_id, unit).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_unit_record(pool: &PgPool, unit_id: i32, unit: Unit) -> Result<(), AppError> {
    match sqlx::query!(
        "UPDATE unit SET singular_name = $1, plural_name = $2 WHERE unit_id = $3",
        unit.singular_name,
        unit.plural_name,
        unit_id,
    )
    .execute(pool)
    .await
    {
        Ok(result) if result.rows_affected() == 0 => Err(AppError::NotFound),
        Err(_) => Err(AppError::InternalServerError),
        _ => Ok(()),
    }
}

pub async fn get_unit(
    State(app_state): State<AppState>,
    Path(unit_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let unit = fetch_unit_from_db(&app_state.pool, unit_id).await?;
    Ok(Json(unit))
}
async fn fetch_unit_from_db(pool: &PgPool, unit_id: i32) -> Result<Unit, AppError> {
    let unit = sqlx::query_as!(
        Unit,
        r#"
            SELECT * 
            FROM unit
            WHERE unit_id = $1
        "#,
        unit_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;
    Ok(unit)
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
#[derive(Serialize, Deserialize, Debug)]
pub struct UnitsQuery {
    limit: i64,
    // Default start_id is 0
    #[serde(default)]
    start_from: i32,
}

pub async fn get_units_by_query(
    State(app_state): State<AppState>,
    query: Query<UnitsQuery>,
) -> Result<impl IntoResponse, AppError> {
    if query.limit > 15 || query.limit < 1 {
        return Err(AppError::BadRequest);
    }
    let mut units: Vec<Unit> = fetch_units_from_db(&query, &app_state.pool).await?;
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

async fn fetch_units_from_db(
    query: &Query<UnitsQuery>,
    pool: &PgPool,
) -> Result<Vec<Unit>, AppError> {
    let result = sqlx::query_as!(
        Unit,
        r#" SELECT * 
            FROM unit
            WHERE unit_id >= $1
            ORDER BY unit_id
            LIMIT $2
        "#,
        query.start_from,
        query.limit + 1,
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}
