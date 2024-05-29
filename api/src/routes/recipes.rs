use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    application::{error::AppError, state::AppState},
    recipe::{
        helpers::{
            bulk_insert_recipe_ingredients, bulk_insert_steps, delete_recipe_ingredients,
            delete_recipe_steps, insert_recipe, update_recipe,
        },
        recipe::{Backed, NotBacked, Recipe},
        recipe_ingredient::{CompactRecipeIngredient, DetailedRecipeIngredient},
    },
    utilities::{fetchers::fetch_recipe_detailed, queries::PaginationQuery},
};
#[instrument]
pub async fn add_recipe_handler(
    State(app_state): State<AppState>,
    Json(recipe): Json<Recipe<CompactRecipeIngredient, NotBacked>>,
) -> Result<StatusCode, AppError> {
    let recipe = recipe.validate()?;
    info!("Beginning transaction");
    let mut transaction = app_state.pool.begin().await?;
    info!("Inserting recipe to db");
    let recipe_id = insert_recipe(&recipe, &mut *transaction).await?;
    info!("Inserting ingredients to db");
    bulk_insert_recipe_ingredients(recipe.ingredients(), recipe_id, &mut *transaction).await?;
    info!("Inserting steps to db");
    bulk_insert_steps(recipe.steps(), recipe_id, &mut *transaction).await?;
    info!("Committing transaction");
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct RemoveRecipeRequest {
    pub recipe_id: i32,
}
// NOTE (oliver): Deleting a recipe_id will cascade on a database level.
// NOTE (oliver): That is why only that is deleted.
pub async fn remove_recipe_handler(
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

pub async fn update_recipe_handler(
    State(app_state): State<AppState>,
    Path(recipe_id): Path<i32>,
    Json(recipe): Json<Recipe<CompactRecipeIngredient, NotBacked>>,
) -> Result<StatusCode, AppError> {
    let recipe: Recipe<CompactRecipeIngredient, Backed> =
        recipe.to_backed(&app_state.unit_ids, &app_state.ingredient_ids)?;
    let mut transaction = app_state.pool.begin().await?;
    update_recipe(recipe_id, recipe.name(), recipe.description(), &mut *transaction).await?;
    delete_recipe_ingredients(recipe_id, &app_state).await?;
    delete_recipe_steps(recipe_id, &app_state).await?;
    bulk_insert_recipe_ingredients(recipe.ingredients(), recipe_id, &mut *transaction).await?;
    bulk_insert_steps(recipe.steps(), recipe_id, &mut *transaction).await?;
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_recipe_handler(
    State(app_state): State<AppState>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<Recipe<DetailedRecipeIngredient, Backed>>, AppError> {
    let recipe = fetch_recipe_detailed(&app_state.pool, recipe_id).await?;
    Ok(Json(recipe))
}

#[derive(Serialize, Deserialize)]
pub struct GetRecipesResponse {
    pub recipes: Vec<Recipe<DetailedRecipeIngredient, Backed>>,
    pub next_start_from: Option<i32>,
}

pub async fn get_recipe_by_query_handler(
    State(state): State<AppState>,
    query: Query<PaginationQuery>,
) -> Result<Json<GetRecipesResponse>, AppError> {
    if query.limit > 15 || query.limit < 1 {
        return Err(AppError::BadRequest);
    }
    let recipe_ids = sqlx::query!(
        r#"
                SELECT recipe_id as id 
                FROM recipe
                WHERE recipe_id >= $1 
                ORDER BY recipe_id
                LIMIT $2;
            "#,
        query.start_from,
        query.limit + 1
    )
    .fetch_all(&state.pool)
    .await?;
    // OPTIMIZE (oliver): This could be faster if it used arrays, as the max capacity is known.
    let mut recipes: Vec<Recipe<DetailedRecipeIngredient, Backed>> = Vec::new();

    for recipe_id_record in recipe_ids {
        let recipe = fetch_recipe_detailed(&state.pool, recipe_id_record.id).await?;
        recipes.push(recipe);
    }

    let next_start_from: Option<i32> = {
        // We are casting length upwards so that it is not lossy.
        // It is (<=) because the vector we have in ingredients
        // is always going to try to fetch 1 more ingredient.
        if (recipes.len() as i64) < query.limit + 1 {
            None
        } else {
            // Remove one element as the fetching overallocated by 1
            recipes
                .pop()
                .and_then(|rec: Recipe<DetailedRecipeIngredient, Backed>| rec.recipe_id())
        }
    };
    let response = GetRecipesResponse {
        recipes,
        next_start_from,
    };
    Ok(Json(response))
}
