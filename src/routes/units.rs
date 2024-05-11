use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};

use crate::{
    app::{AppError, AppState},
    unit::Unit,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveUnitRequest {
    pub unit_id: i32,
}
pub async fn add_unit(
    State(app_state): State<AppState>,
    Json(unit): Json<Unit>,
) -> Result<StatusCode, AppError> {
    insert_unit_into_db(&unit, &app_state.pool).await?;
    cache_unit_id(unit.unit_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}

async fn insert_unit_into_db(unit: &Unit, pool: &PgPool) -> Result<(), AppError> {
    match query!(
        r#"
            INSERT INTO unit (singular_name, plural_name) 
            VALUES ($1, $2);
        "#,
        unit.singular_name,
        unit.plural_name
    )
    .execute(pool)
    .await
    {
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Err(AppError::Conflict),
        Err(_) => Err(AppError::InternalServerError),
        Ok(_) => Ok(()),
    }
}

fn cache_unit_id(unit_id: i32, app_state: AppState) {
    app_state.unit_ids.insert(unit_id);
}
fn remove_unit_id_from_cache(unit_id: &i32, app_state: AppState) {
    app_state.unit_ids.remove(unit_id);
}
// TODO (oliver): Removing non_existent_unit_id
pub async fn remove_unit(
    State(app_state): State<AppState>,
    Json(delete_unit_request): Json<RemoveUnitRequest>,
) -> Result<StatusCode, AppError> {
    delete_unit_from_db(&delete_unit_request.unit_id, &app_state.pool).await?;
    remove_unit_id_from_cache(&delete_unit_request.unit_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}
async fn delete_unit_from_db(unit_id: &i32, pool: &PgPool) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM unit WHERE unit_id = $1",
        unit_id
    )
    .execute(pool)
    .await?;
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
