use crate::application::error::AppError;
use sqlx::{query, Executor, PgPool, Postgres};

use super::Unit;

/// Updates a unit in the database by its ID.
///
/// This function updates the singular and plural names of a unit with the specified unit ID in the database.
/// If the unit with the specified ID is not found, it returns an `AppError::NotFound`.
///
/// # Parameters
/// - `executor`: An executor that implements `Executor` for running the query. This can be a connection pool, a connection, or a transaction.
/// - `unit_id`: The ID of the unit to update.
/// - `unit`: A `Unit` instance containing the updated unit details.
///
/// # Returns
/// - `Result<(), AppError>`: Returns `Ok(())` if the update is successful,
///   or an `AppError` if an error occurs during the update.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to update the unit in the database fails.
/// - The unit with the specified ID is not found.
pub async fn update_unit(
    executor: impl Executor<'_, Database = Postgres>,
    unit_id: i32,
    unit: &Unit,
) -> Result<(), AppError> {
    match sqlx::query!(
        "UPDATE unit SET singular_name = $1, plural_name = $2 WHERE unit_id = $3",
        unit.singular_name,
        unit.plural_name,
        unit_id,
    )
    .execute(executor)
    .await
    {
        Ok(result) if result.rows_affected() == 0 => Err(AppError::NotFound),
        Err(_) => Err(AppError::InternalServerError),
        _ => Ok(()),
    }
}
/// Deletes a unit from the database by its ID.
///
/// This function deletes a unit with the specified unit ID from the database.
/// If the unit with the specified ID is not found, it returns an `AppError::NotFound`.
///
/// # Parameters
/// - `unit_id`: A reference to the ID of the unit to delete.
/// - `executor`: An executor that implements `Executor` for running the query. This can be a connection pool, a connection, or a transaction.
///
/// # Returns
/// - `Result<(), AppError>`: Returns `Ok(())` if the deletion is successful,
///   or an `AppError` if an error occurs during the deletion.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to delete the unit from the database fails.
/// - The unit with the specified ID is not found.
pub async fn delete_unit(
    unit_id: &i32,
    executor: impl Executor<'_, Database = Postgres>,
) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM unit WHERE unit_id = $1", unit_id)
        .execute(executor)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

/// Inserts a unit into the database.
///
/// This function inserts a new unit into the database. The unit details are provided as a `Unit` instance,
/// and the function returns the ID of the newly inserted unit. If the unit already exists (based on unique constraints),
/// it returns an `AppError::Conflict`.
///
/// # Parameters
/// - `unit`: A reference to a `Unit` instance containing the unit details.
/// - `executor`: An executor that implements `Executor` for running the query. This can be a connection pool, a connection, or a transaction.
///
/// # Returns
/// - `Result<i32, AppError>`: A result containing the ID of the newly inserted unit if the insertion is successful,
///   or an `AppError` if an error occurs during the insertion.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to insert the unit into the database fails.
/// - The unit already exists (based on unique constraints).
pub async fn insert_unit(
    unit: &Unit,
    executor: impl Executor<'_, Database = Postgres>,
) -> Result<i32, AppError> {
    match query!(
        r#"
            INSERT INTO unit (singular_name, plural_name) 
            VALUES ($1, $2)
            RETURNING unit_id;
        "#,
        unit.singular_name,
        unit.plural_name
    )
    .fetch_one(executor)
    .await
    {
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Err(AppError::Conflict),
        Err(_) => Err(AppError::InternalServerError),
        Ok(rec) => Ok(rec.unit_id),
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use sqlx::{query, PgPool};

    use crate::{
        unit::{
            helpers::{delete_unit, insert_unit, update_unit},
            Unit,
        },
        utilities::random_generation::units::choose_random_unit,
    };

    #[sqlx::test]
    async fn test_insert_unit(pool: PgPool) -> sqlx::Result<()> {
        let new_unit = Unit {
            unit_id: None,
            singular_name: Faker.fake::<String>(),
            plural_name: "Test Units".to_string(),
        };

        // Call the function to insert the unit
        let unit_id = insert_unit(&new_unit, &pool).await.unwrap();

        // Verify that the unit has been inserted
        let inserted_unit = query!(
            "SELECT singular_name, plural_name FROM unit WHERE unit_id = $1",
            unit_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(inserted_unit.singular_name, new_unit.singular_name);
        assert_eq!(inserted_unit.plural_name, new_unit.plural_name);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("units")))]
    async fn test_update_unit(pool: PgPool) -> sqlx::Result<()> {
        let unit_id = choose_random_unit(&pool)
            .await
            .unit_id
            .expect("should have found unit in the database");
        let updated_unit = Unit {
            unit_id: Some(unit_id),
            singular_name: Faker.fake::<String>(),
            plural_name: Faker.fake::<String>(),
        };

        // Call the function to update the unit
        update_unit(&pool, unit_id, &updated_unit).await.unwrap();

        // Verify that the unit has been updated
        let updated_record = query!(
            "SELECT singular_name, plural_name FROM unit WHERE unit_id = $1",
            unit_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(updated_record.singular_name, updated_unit.singular_name);
        assert_eq!(updated_record.plural_name, updated_unit.plural_name);

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("units")))]
    async fn test_delete_unit(pool: PgPool) -> sqlx::Result<()> {
        let unit_id = choose_random_unit(&pool)
            .await
            .unit_id
            .expect("should have found unit in the database");

        // Call the function to delete the unit
        delete_unit(&unit_id, &pool).await.unwrap();

        // Verify that the unit has been deleted
        let deleted_unit = query!("SELECT unit_id FROM unit WHERE unit_id = $1", unit_id)
            .fetch_optional(&pool)
            .await?;

        assert!(deleted_unit.is_none());

        Ok(())
    }
}
