use sqlx::{postgres::PgQueryResult, Executor, PgPool, Postgres};

use crate::application::{
    error::{AppError, RecipeParsingError},
    state::AppState,
};

use super::{
    recipe::{Backed, Recipe},
    recipe_ingredient::{CompactRecipeIngredient, RecipeIngredient},
    recipe_step::RecipeStep,
};
type SqlxError = sqlx::Error;

/// Inserts a recipe into the database.
///
/// This function inserts a new recipe into the database. The recipe is provided as a `Recipe` instance,
/// and the function returns the ID of the newly inserted recipe.
///
/// # Parameters
/// - `recipe`: A reference to a `Recipe<I, BackedState>` instance containing the recipe details.
/// - `transaction`: A mutable reference to a SQL transaction.
///
/// # Returns
/// - `Result<i32, AppError>`: A result containing the ID of the newly inserted recipe if the insertion is successful,
///   or an `AppError` if an error occurs during the insertion.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to insert the recipe into the database fails.
pub async fn insert_recipe<I: RecipeIngredient, BackedState>(
    recipe: &Recipe<I, BackedState>,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<i32, AppError> {
    let recipe_query_result = sqlx::query!(
        r#"
            INSERT INTO recipe (name, description) VALUES ($1, $2) RETURNING recipe_id
        "#,
        recipe.name(),
        recipe.description()
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(recipe_query_result.recipe_id)
}

/// Bulk inserts recipe ingredients into the database.
///
/// This function inserts multiple ingredients into the database for a specified recipe ID in a single operation.
/// The ingredients are provided as a slice of `CompactRecipeIngredient` instances, and the function returns
/// a result indicating the success or failure of the operation.
///
/// # Parameters
/// - `ingredients`: A slice of `CompactRecipeIngredient` instances containing the ingredients to be inserted.
/// - `recipe_id`: The ID of the recipe to which the ingredients belong.
/// - `transaction`: A mutable reference to a SQL transaction.
///
/// # Returns
/// - `Result<(), AppError>`: A result indicating success (`Ok(())`) or an error (`AppError`) if the insertion fails.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to insert the ingredients into the database fails.
/// - There is a foreign key violation (invalid ingredient ID).
/// - There is a unique constraint violation (duplicate ingredient ID).
pub async fn bulk_insert_ingredients(
    ingredients: &[CompactRecipeIngredient],
    recipe_id: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> sqlx::Result<(), AppError> {
    let ingr_ids: Vec<i32> = ingredients
        .iter()
        .map(|ingr| ingr.ingredient().clone())
        .collect(); // Clone here will actually just 'Copy'
    let unit_ids: Vec<i32> = ingredients.iter().map(|ingr| ingr.unit().clone()).collect(); // Clone here will actually just 'Copy'
    let quants: Vec<String> = ingredients
        .iter()
        .map(|ingr| ingr.quantity().to_owned())
        .collect();
    // Recipe id is always the same, so we can just do that.
    let rec_ids: Vec<i32> = (0..ingr_ids.len()).map(|_| recipe_id).collect();

    match sqlx::query!(
        r#"
            INSERT INTO recipe_ingredient (recipe_id, ingredient_id, unit_id, quantity)
            SELECT * 
            FROM UNNEST($1::INT[], $2::INT[], $3::INT[], $4::VARCHAR(50)[]);
        "#,
        &rec_ids,
        &ingr_ids,
        &unit_ids,
        &quants
    )
    .execute(&mut **transaction)
    .await
    {
        Ok(_) => Ok(()),
        Err(SqlxError::Database(db_err)) if db_err.is_foreign_key_violation() => Err(
            AppError::RecipeParsingError(RecipeParsingError::InvalidIngredientId),
        ),
        Err(SqlxError::Database(db_err)) if db_err.is_unique_violation() => Err(
            AppError::RecipeParsingError(RecipeParsingError::DuplicateIngredientId),
        ),
        Err(_) => Err(AppError::InternalServerError),
    }
}

/// Bulk inserts steps into the database for a given recipe.
///
/// This function inserts multiple steps into the database for a specified recipe ID in a single operation.
/// The steps are provided as a slice of `RecipeStep` instances, and the function returns the result of the query execution.
///
/// # Parameters
/// - `steps`: A slice of `RecipeStep` instances containing the steps to be inserted.
/// - `recipe_id`: The ID of the recipe to which the steps belong.
/// - `transaction`: A mutable reference to a SQL transaction.
///
/// # Returns
/// - `Result<PgQueryResult, sqlx::Error>`: A result containing the `PgQueryResult` if the insertion is successful,
///   or a `sqlx::Error` if an error occurs during the insertion.
///
/// # Errors
/// This function returns a `sqlx::Error` if:
/// - The query to insert the steps into the database fails.
pub async fn bulk_insert_steps(
    steps: &[RecipeStep],
    recipe_id: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    let step_numbers: Vec<i32> = steps.iter().map(|step| step.step_number).collect();
    let instructions: Vec<String> = steps.iter().map(|step| step.instruction.clone()).collect();
    let rec_ids: Vec<i32> = (0..step_numbers.len()).map(|_| recipe_id).collect();

    let query_result = sqlx::query!(
        r#"
                INSERT INTO step (recipe_id, step_number, instruction)
                SELECT * FROM UNNEST($1::INT[], $2::INT[], $3::TEXT[]);
            "#,
        &rec_ids,
        &step_numbers,
        &instructions
    )
    .execute(&mut **transaction)
    .await?;
    Ok(query_result)
}

/// Updates a recipe in the database.
///
/// This function updates the name and description of a recipe with the specified recipe ID in the database.
/// If the recipe with the specified ID is not found, it returns an `AppError::NotFound`.
///
/// # Parameters
/// - `recipe_id`: The ID of the recipe to update.
/// - `name`: A reference to the new name for the recipe.
/// - `description`: A reference to the new description for the recipe.
/// - `executor`: An executor that implements `Executor` for running the query. This can be a connection pool, a connection, or a transaction.
///
/// # Returns
/// - `Result<(), AppError>`: Returns `Ok(())` if the update is successful,
///   or an `AppError` if an error occurs during the update.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to update the recipe in the database fails.
/// - The recipe with the specified ID is not found.
pub async fn update_recipe(
    recipe_id: i32,
    name: &str,
    description: &str,
    executor: impl Executor<'_, Database = Postgres>,
) -> Result<(), AppError> {
    let recipe_query_result = sqlx::query!(
        r#"
            UPDATE recipe
            SET name = $1, description = $2 
            WHERE recipe_id = $3 AND (name IS DISTINCT FROM $1 OR description IS DISTINCT FROM $2)
        "#,
        name,
        description,
        recipe_id
    )
    .execute(executor)
    .await?;
    if recipe_query_result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

/// Deletes all steps associated with a given recipe ID.
///
/// This function deletes all steps in the database associated with the specified recipe ID.
/// If the operation is successful, it returns `Ok(())`. If an error occurs during the operation,
/// it returns an `AppError`.
///
/// # Parameters
/// - `recipe_id`: The ID of the recipe whose steps are to be deleted.
/// - `app_state`: A reference to the application state containing the PostgreSQL connection pool.
///
/// # Returns
/// - `Result<(), AppError>`: Returns `Ok(())` if the deletion is successful,
///   or an `AppError` if an error occurs during the deletion.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to delete the steps from the database fails.
pub async fn delete_recipe_steps(recipe_id: i32, app_state: &AppState) -> Result<(), AppError> {
    sqlx::query!(
        r#"
            DELETE FROM step 
            WHERE recipe_id = $1
        "#,
        recipe_id
    )
    .execute(&app_state.pool)
    .await?;
    Ok(())
}

/// Deletes all recipe_ingredients associated with a given recipe ID.
///
/// This function deletes all recipe_ingredients in the database associated with the specified recipe ID.
/// If the operation is successful, it returns `Ok(())`. If an error occurs during the operation,
/// it returns an `AppError`.
///
/// # Parameters
/// - `recipe_id`: The ID of the recipe whose recipe_ingredients are to be deleted.
/// - `app_state`: A reference to the application state containing the PostgreSQL connection pool.
///
/// # Returns
/// - `Result<(), AppError>`: Returns `Ok(())` if the deletion is successful,
///   or an `AppError` if an error occurs during the deletion.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to delete the recipe_ingredients from the database fails.
pub async fn delete_recipe_ingredients(
    recipe_id: i32,
    app_state: &AppState,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
            DELETE FROM recipe_ingredient 
            WHERE recipe_id = $1
        "#,
        recipe_id
    )
    .execute(&app_state.pool)
    .await?;
    Ok(())
}

#[allow(unused)]
mod test {
    use fake::{Fake, Faker};
    use sqlx::PgPool;

    use crate::{
        application::state::AppState,
        recipe::{
            self,
            helpers::{delete_recipe_ingredients, delete_recipe_steps, update_recipe},
            recipe::Recipe,
        },
        utilities::random_generation::recipes::choose_random_recipe_id,
    };

    #[sqlx::test(fixtures(
        path = "../../tests/fixtures",
        scripts("recipes", "units", "ingredients", "recipe_ingredients")
    ))]
    async fn test_delete_recipe_ingredients(pool: PgPool) -> sqlx::Result<()> {
        let app_state = AppState::new(pool.clone());
        let recipe_id = choose_random_recipe_id(&pool).await;
        delete_recipe_ingredients(recipe_id, &app_state)
            .await
            .unwrap();
        // Verify that the ingredient has been deleted
        let ingredients_after = sqlx::query!(
            "SELECT * FROM recipe_ingredient WHERE recipe_id = $1",
            recipe_id
        )
        .fetch_optional(&app_state.pool)
        .await
        .unwrap();
        assert!(ingredients_after.is_none());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("recipes", "steps")))]
    async fn test_delete_recipe_steps(pool: PgPool) -> sqlx::Result<()> {
        let app_state = AppState::new(pool.clone());
        let recipe_id = choose_random_recipe_id(&pool).await;
        delete_recipe_steps(recipe_id, &app_state).await.unwrap();
        // Verify that the ingredient has been deleted
        let steps_after = sqlx::query!("SELECT * FROM step WHERE recipe_id = $1", recipe_id)
            .fetch_optional(&app_state.pool)
            .await
            .unwrap();
        assert!(steps_after.is_none());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../../tests/fixtures", scripts("recipes")))]
    async fn test_update_recipe(pool: PgPool) -> sqlx::Result<()> {
        let mut transaction = pool.begin().await?;
        let recipe_id = choose_random_recipe_id(&pool).await;
        let new_name = Faker.fake::<String>();
        let new_description = Faker.fake::<String>();
        update_recipe(recipe_id, &new_name, &new_description, &mut *transaction)
            .await
            .unwrap();
        transaction.commit().await?;
        // Verify that the recipe has been updated
        let updated_record = sqlx::query!(
            "SELECT name, description FROM recipe WHERE recipe_id = $1",
            recipe_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(updated_record.name, new_name);
        assert_eq!(updated_record.description, new_description);

        Ok(())
    }
}
