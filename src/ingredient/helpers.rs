use sqlx::{query, PgPool};

use crate::application::error::AppError;

use super::Ingredient;

/// Inserts a new ingredient into the database.
///
/// This function inserts a new ingredient into the database. The ingredient details are provided
/// as an `Ingredient` instance, and the function returns the ID of the newly inserted ingredient.
/// If the ingredient already exists (based on unique constraints), it returns an `AppError::Conflict`.
///
/// # Parameters
/// - `ingredient`: A reference to an `Ingredient` instance containing the ingredient details.
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `Result<i32, AppError>`: A result containing the ID of the newly inserted ingredient if the insertion is successful,
///   or an `AppError::Conflict` if the ingredient already exists, or another `AppError` if an error occurs during the insertion.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to insert the ingredient into the database fails.
/// - The ingredient already exists (based on unique constraints).
pub async fn insert_ingredient(
    ingredient: &Ingredient,
    pool: &PgPool,
) -> Result<i32, AppError> {
    match query!(
        r#"
            INSERT INTO ingredient (singular_name, plural_name) 
            VALUES ($1, $2)
            RETURNING ingredient_id;
        "#,
        ingredient.singular_name,
        ingredient.plural_name
    )
    .fetch_one(pool)
    .await
    {
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Err(AppError::Conflict),
        Err(_) => Err(AppError::InternalServerError),
        Ok(rec) => Ok(rec.ingredient_id),
    }
}

/// Updates an ingredient in the database by its ID.
///
/// This function updates the singular and plural names of an ingredient with the specified ingredient ID in the database.
/// If the ingredient with the specified ID is not found, it returns an `AppError::NotFound`.
///
/// # Parameters
/// - `ingredient`: An `Ingredient` instance containing the updated ingredient details.
/// - `ingredient_id`: The ID of the ingredient to update.
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `Result<(), AppError>`: Returns `Ok(())` if the update is successful,
///   or an `AppError` if an error occurs during the update.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to update the ingredient in the database fails.
/// - The ingredient with the specified ID is not found.
pub async fn update_ingredient(
    ingredient: Ingredient,
    ingredient_id: i32,
    pool: &PgPool,
) -> Result<(), AppError> {
    match sqlx::query!(
        "UPDATE ingredient SET singular_name = $1, plural_name = $2 WHERE ingredient_id = $3",
        ingredient.singular_name,
        ingredient.plural_name,
        ingredient_id,
    )
    .execute(pool)
    .await
    {
        Ok(result) if result.rows_affected() == 0 => Err(AppError::NotFound),
        Err(_) => Err(AppError::InternalServerError),
        _ => Ok(()),
    }
}

/// Deletes an ingredient from the database by its ID.
///
/// This function deletes an ingredient with the specified ingredient ID from the database.
/// If the ingredient with the specified ID is not found, it returns an `AppError::NotFound`.
///
/// # Parameters
/// - `ingredient_id`: A reference to the ID of the ingredient to delete.
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `Result<(), AppError>`: Returns `Ok(())` if the deletion is successful,
///   or an `AppError` if an error occurs during the deletion.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to delete the ingredient from the database fails.
/// - The ingredient with the specified ID is not found.
pub async fn delete_ingredient(ingredient_id: &i32, pool: &PgPool) -> Result<(), AppError> {
    let result = sqlx::query!(
        "DELETE FROM ingredient WHERE ingredient_id = $1",
        ingredient_id
    )
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}