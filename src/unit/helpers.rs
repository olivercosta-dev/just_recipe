use sqlx::{query, PgPool};
use crate::application::error::AppError;

use super::Unit;

/// Updates a unit in the database.
///
/// This function updates the singular and plural names of a unit with the specified unit ID in the database.
/// If the unit with the specified ID is not found, it returns an `AppError::NotFound`.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
/// - `unit_id`: The ID of the unit to update.
/// - `unit`: A `Unit` instance containing the updated unit details.
///
/// # Returns
/// - `Result<(), AppError>`: Returns `Ok(())` if the update is successful,
///   or an `AppError` if an error occurs during the update.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to update the unit fails.
/// - The unit with the specified ID is not found.
pub async fn update_unit(pool: &PgPool, unit_id: i32, unit: Unit) -> Result<(), AppError> {
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

pub async fn delete_unit(unit_id: &i32, pool: &PgPool) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM unit WHERE unit_id = $1", unit_id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}


pub async fn insert_unit(unit: &Unit, pool: &PgPool) -> Result<i32, AppError> {
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
