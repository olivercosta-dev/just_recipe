use sqlx::{postgres::PgQueryResult, PgPool};

use crate::{application::{error::{AppError, RecipeParsingError}, state::AppState}, ingredient::Ingredient, unit::Unit};

use super::{recipe::{Backed, Recipe}, recipe_ingredient::{CompactRecipeIngredient, DetailedRecipeIngredient, RecipeIngredient}, recipe_step::RecipeStep};
type SqlxError = sqlx::Error;

// INSERT
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
// TODO (oliver): Unit test the utility functions
pub async fn bulk_insert_ingredients(
    ingredients: &[CompactRecipeIngredient],
    recipe_id: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> sqlx::Result<(), AppError> {
    // OPTIMIZE (oliver): There is lot of cloning and things going on because
    // OPTIMIZE (oliver): PgArray seems to want ownership. There is probably a solution to this.

    let ingr_ids: Vec<i32> = ingredients
        .iter()
        .map(|ingr| ingr.ingredient().to_owned())
        .collect();
    let unit_ids: Vec<i32> = ingredients
        .iter()
        .map(|ingr| ingr.unit().to_owned())
        .collect();
    let quants: Vec<String> = ingredients
        .iter()
        .map(|ingr| ingr.quantity().to_owned())
        .collect();
    let rec_ids: Vec<i32> = (0..ingr_ids.len()).map(|_| recipe_id).collect();

    match sqlx::query!(
        r#"
            INSERT INTO recipe_ingredient (recipe_id, ingredient_id, unit_id, quantity)
            SELECT * FROM UNNEST($1::INT[], $2::INT[], $3::INT[], $4::VARCHAR(50)[]);
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

// TODO (oliver): Make this general so that even non-detailed recipes can be fetched!
pub async fn fetch_recipe_from_db(
    pool: &PgPool,
    recipe_id: i32,
) -> Result<Recipe<DetailedRecipeIngredient, Backed>, AppError> {
    let (name, description) = {
        let record = sqlx::query!(
            r#"
            SELECT name, description
            FROM recipe
            WHERE recipe_id = $1
        "#,
            recipe_id
        )
        .fetch_optional(pool)
        .await?;
        if record.is_none() {
            return Err(AppError::NotFound);
        }
        let record = record.unwrap();
        (record.name, record.description)
    };
    let recipe_ingredient_records = sqlx::query!(
        r#"
            SELECT 
                i.ingredient_id, 
                i.singular_name,
                i.plural_name,
                u.unit_id,
                u.singular_name as unit_singular,
                u.plural_name as unit_plural,
                quantity
            FROM recipe_ingredient ri
            LEFT JOIN ingredient i
            ON ri.ingredient_id = i.ingredient_id
            LEFT JOIN unit u
            ON ri.unit_id = u.unit_id
            WHERE recipe_id = $1
        "#,
        recipe_id
    )
    .fetch_all(pool)
    .await?;

    let mut detailed_ingredients: Vec<DetailedRecipeIngredient> = Vec::new();

    for record in recipe_ingredient_records {
        let ingredient = Ingredient {
            ingredient_id: Some(record.ingredient_id),
            singular_name: record.singular_name,
            plural_name: record.plural_name,
        };
        let unit = Unit {
            unit_id: Some(record.unit_id),
            singular_name: record.unit_singular,
            plural_name: record.unit_plural,
        };
        let detailed_ingredient =
            DetailedRecipeIngredient::new(recipe_id, ingredient, unit, record.quantity);
        detailed_ingredients.push(detailed_ingredient);
    }
    let steps = sqlx::query_as!(
        RecipeStep,
        r#"
            SELECT step_id, recipe_id, step_number, instruction
            FROM step
            WHERE recipe_id = $1
            ORDER BY step_number
        "#,
        recipe_id
    )
    .fetch_all(pool)
    .await?;
    let recipe = Recipe::new(recipe_id, name, description, detailed_ingredients, steps);
    Ok(recipe)
}

// UPDATE
pub async fn update_recipe_record<I: RecipeIngredient>(
    recipe: &Recipe<I, Backed>,
    recipe_id: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), AppError> {
    let recipe_query_result = sqlx::query!(
        r#"
            UPDATE recipe
            SET name = $1, description = $2 
            WHERE recipe_id = $3 AND (name IS DISTINCT FROM $1 OR description IS DISTINCT FROM $2)
        "#,
        recipe.name(),
        recipe.description(),
        recipe_id
    )
    .execute(&mut **transaction)
    .await?;
    if recipe_query_result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

// DELETE
// Delete all the step records for a given recipe_id
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

// Delete all the recipe_ingredient records for a given recipe_id
pub async fn delete_recipe_ingredients(recipe_id: i32, app_state: &AppState) -> Result<(), AppError> {
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