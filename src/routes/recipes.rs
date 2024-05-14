use crate::{app::*, recipe::*};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use sqlx::{postgres::PgQueryResult, Error as SqlxError, PgPool};

#[derive(Deserialize)]
pub struct RemoveRecipeRequest {
    pub recipe_id: i32,
}

/* Example request:
{
    "name": "Very Tasty Soup",
    "description": "Finger-licking Good!",
    "ingredients": [
        {
            "ingredient_id": 1,
            "unit_id": 1,
            "quantity": "3/4",
        },
        {
            "ingredient_id": 1,
            "unit_id": 2,
            "quantity": "1/2",
        }
    ],
    "steps": [
        {
            "step_number": 1,
            "instruction": "Put the apple in boiling hot water."
        },
        {
            "step_number": 2,
            "instruction": "Eat the apple."
        }
    ]
}
*/
pub async fn add_recipe(
    State(app_state): State<AppState>,
    Json(unchecked_recipe): Json<UncheckedRecipe>,
) -> Result<StatusCode, AppError> {
    let recipe: CompressedRecipe = unchecked_recipe.try_into()?;
    let mut transaction = app_state.pool.begin().await?;
    let recipe_id = insert_recipe(&recipe, &mut transaction).await?;
    bulk_insert_ingredients(recipe.ingredients, recipe_id, &mut transaction).await?;
    bulk_insert_steps(recipe.steps, recipe_id, &mut transaction).await?;
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn insert_recipe(
    recipe: &CompressedRecipe,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<i32, AppError> {
    let recipe_query_result = sqlx::query!(
        r#"
            INSERT INTO recipe (name, description) VALUES ($1, $2) RETURNING recipe_id
        "#,
        recipe.name,
        recipe.description
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(recipe_query_result.recipe_id)
}

// TODO (oliver): Perhaps unit test the utility functions?
async fn bulk_insert_ingredients(
    ingredients: Vec<CompressedIngredient>,
    recipe_id: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> sqlx::Result<(), AppError> {
    let ingr_ids: Vec<i32> = ingredients.iter().map(|ingr| ingr.ingredient_id).collect();
    let unit_ids: Vec<i32> = ingredients.iter().map(|ingr| ingr.unit_id).collect();
    let quants: Vec<String> = ingredients
        .iter()
        .map(|ingr| ingr.quantity.clone())
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

async fn bulk_insert_steps(
    steps: Vec<RecipeStep>,
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

// TODO (oliver): Remove a non existent recipe
// Deleting a recipe_id will cascade on a database level.
pub async fn remove_recipe(
    State(app_state): State<AppState>,
    Json(remove_recipe_request): Json<RemoveRecipeRequest>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query!(
        "DELETE FROM recipe WHERE recipe_id = $1",
        remove_recipe_request.recipe_id
    )
    .execute(&app_state.pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_recipe(
    State(app_state): State<AppState>,
    Path(recipe_id): Path<i32>,
    Json(unchecked_recipe): Json<UncheckedRecipe>,
) -> Result<StatusCode, AppError> {
    let recipe: CompressedRecipe = CompressedRecipe::parse(
        unchecked_recipe,
        &app_state.unit_ids,
        &app_state.ingredient_ids,
    )?;
    let mut transaction = app_state.pool.begin().await?;
    update_recipe_record(&recipe, recipe_id, &mut transaction).await?;

    delete_recipe_ingredients(recipe_id, &app_state).await?;
    delete_recipe_steps(recipe_id, &app_state).await?;
    bulk_insert_ingredients(recipe.ingredients, recipe_id, &mut transaction).await?;
    bulk_insert_steps(recipe.steps, recipe_id, &mut transaction).await?;
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_recipe_record(
    recipe: &CompressedRecipe,
    recipe_id: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), AppError> {
    let recipe_query_result = sqlx::query!(
        r#"
            UPDATE recipe
            SET name = $1, description = $2 
            WHERE recipe_id = $3 AND (name IS DISTINCT FROM $1 OR description IS DISTINCT FROM $2)
        "#,
        recipe.name,
        recipe.description,
        recipe_id
    )
    .execute(&mut **transaction)
    .await?;
    if recipe_query_result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

// Delete all the step records for a given recipe_id
async fn delete_recipe_steps(recipe_id: i32, app_state: &AppState) -> Result<(), AppError> {
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
async fn delete_recipe_ingredients(recipe_id: i32, app_state: &AppState) -> Result<(), AppError> {
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

pub async fn get_recipe(
    State(app_state): State<AppState>,
    Path(recipe_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let recipe = fetch_recipe_from_db(&app_state.pool, recipe_id).await?;
    let detailed = CompressedRecipe::parse_detailed(recipe, &app_state.pool).await?; 
    Ok(Json(detailed))
}

async fn fetch_recipe_from_db(pool: &PgPool, recipe_id: i32) -> Result<CompressedRecipe, AppError> {
    let (name, description) = {
        
        let optional_record = sqlx::query!(
            r#"
            SELECT name, description
            FROM recipe
            WHERE recipe_id = $1
        "#,
            recipe_id
        )
        .fetch_optional(pool)
        .await?;
        if optional_record.is_none() {
            return Err(AppError::NotFound);
        }
        let record = optional_record.unwrap();
        (record.name, record.description)
    };
    let ingredients = sqlx::query_as!(
        CompressedIngredient,
        r#"
            SELECT recipe_id, ingredient_id, unit_id, quantity
            FROM recipe_ingredient
            WHERE recipe_id = $1
        "#,
        recipe_id
    )
    .fetch_all(pool)
    .await?;
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
    Ok(CompressedRecipe {
        recipe_id,
        name,
        description,
        ingredients,
        steps,
    })
}
