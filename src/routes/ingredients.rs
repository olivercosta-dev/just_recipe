use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};

use crate::{
    application::{error::AppError, state::AppState},
    ingredient::Ingredient,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveIngredientRequest {
    pub ingredient_id: i32,
}
pub async fn add_ingredient(
    State(app_state): State<AppState>,
    Json(ingredient): Json<Ingredient>,
) -> Result<StatusCode, AppError> {
    let ingredient_id = insert_ingredient_into_db(&ingredient, &app_state.pool).await?;
    cache_ingredient_id(ingredient_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}

fn cache_ingredient_id(ingredient_id: i32, app_state: AppState) {
    app_state.ingredient_ids.insert(ingredient_id);
}

fn remove_ingredient_id_from_cache(ingredient_id: &i32, app_state: AppState) {
    app_state.ingredient_ids.remove(ingredient_id);
}

async fn insert_ingredient_into_db(
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

pub async fn remove_ingredient(
    State(app_state): State<AppState>,
    Json(delete_ingredient_request): Json<RemoveIngredientRequest>,
) -> Result<StatusCode, AppError> {
    delete_ingredient_from_db(&delete_ingredient_request.ingredient_id, &app_state.pool).await?;
    remove_ingredient_id_from_cache(&delete_ingredient_request.ingredient_id, app_state);
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_ingredient_from_db(ingredient_id: &i32, pool: &PgPool) -> Result<(), AppError> {
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

pub async fn update_ingredient(
    State(app_state): State<AppState>,
    Path(ingredient_id): Path<i32>,
    Json(ingredient): Json<Ingredient>,
) -> Result<StatusCode, AppError> {
    insert_ingredient_record(ingredient, ingredient_id, &app_state.pool).await?;
    Ok(StatusCode::NO_CONTENT)
}
async fn insert_ingredient_record(
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

pub async fn get_ingredient(
    State(app_state): State<AppState>,
    Path(ingredient_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let ingredient = fetch_ingredient_from_db(&app_state.pool, ingredient_id).await?;
    Ok(Json(ingredient))
}
async fn fetch_ingredient_from_db(
    pool: &PgPool,
    ingredient_id: i32,
) -> Result<Ingredient, AppError> {
    let ingredient = sqlx::query_as!(
        Ingredient,
        r#"
            SELECT * 
            FROM ingredient
            WHERE ingredient_id = $1
        "#,
        ingredient_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;
    Ok(ingredient)
}
