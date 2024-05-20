use sqlx::{query, Executor, PgPool, Postgres};

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
pub async fn insert_ingredient(ingredient: &Ingredient, pool: &PgPool) -> Result<i32, AppError> {
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
/// - `executor`: Something the implements the Executor trait.
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
    executor: impl Executor<'_, Database = Postgres>,
) -> Result<(), AppError> {
    match sqlx::query!(
        "UPDATE ingredient SET singular_name = $1, plural_name = $2 WHERE ingredient_id = $3",
        ingredient.singular_name,
        ingredient.plural_name,
        ingredient_id,
    )
    .execute(executor)
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
#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use sqlx::{query, PgPool};

    use crate::{
        ingredient::{
            helpers::{delete_ingredient, insert_ingredient, update_ingredient},
            Ingredient,
        },
        utilities::random_generation::ingredients::choose_random_ingredient,
    };
    #[sqlx::test]
    async fn test_insert_ingredient(pool: PgPool) -> sqlx::Result<()> {
        let new_ingredient = Ingredient {
            ingredient_id: None,
            singular_name: Faker.fake::<String>(),
            plural_name: "Test Ingredients".to_string(),
        };

        // Call the function to insert the ingredient
        let ingredient_id = insert_ingredient(&new_ingredient, &pool).await.unwrap();

        // Verify that the ingredient has been inserted
        let inserted_ingredient = query!(
            "SELECT singular_name, plural_name FROM ingredient WHERE ingredient_id = $1",
            ingredient_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(
            inserted_ingredient.singular_name,
            new_ingredient.singular_name
        );
        assert_eq!(inserted_ingredient.plural_name, new_ingredient.plural_name);

        Ok(())
    }
    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("ingredients")))]
    async fn test_update_ingredient(pool: PgPool) -> sqlx::Result<()> {
        let ingredient_id = choose_random_ingredient(&pool)
            .await
            .ingredient_id
            .expect("should have found ingredient in the database");
        let updated_ingredient = Ingredient {
            ingredient_id: Some(ingredient_id),
            singular_name: Faker.fake::<String>(),
            plural_name: "Test Ingredients".to_string(),
        };

        // Call the function to update the ingredient
        update_ingredient(updated_ingredient.clone(), ingredient_id, &pool)
            .await
            .unwrap();

        // Verify that the ingredient has been updated
        let updated_record = query!(
            "SELECT singular_name, plural_name FROM ingredient WHERE ingredient_id = $1",
            ingredient_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(
            updated_record.singular_name,
            updated_ingredient.singular_name
        );
        assert_eq!(updated_record.plural_name, updated_ingredient.plural_name);

        Ok(())
    }
    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("ingredients")))]
    async fn test_delete_ingredient(pool: PgPool) -> sqlx::Result<()> {
        let ingredient_id = choose_random_ingredient(&pool)
            .await
            .ingredient_id
            .expect("should have found ingredient in the database");

        // Call the function to delete the ingredient
        delete_ingredient(&ingredient_id, &pool).await.unwrap();

        // Verify that the ingredient has been deleted
        let deleted_ingredient = query!(
            "SELECT ingredient_id FROM ingredient WHERE ingredient_id = $1",
            ingredient_id
        )
        .fetch_optional(&pool)
        .await?;

        assert!(deleted_ingredient.is_none());

        Ok(())
    }
}
