use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query};

use crate::app::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct Ingredient {
    #[serde(skip)]
    pub ingredient_id: i32,
    pub singular_name: String,
    pub plural_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveIngredientRequest {
    pub ingredient_id: i32,
}
pub async fn add_ingredient(
    State(app_state): State<AppState>,
    Json(ingredient): Json<Ingredient>,
) -> StatusCode {
    let result: Result<PgQueryResult, sqlx::Error> = query!(
        r#"
                INSERT INTO ingredient (singular_name, plural_name) 
                VALUES ($1, $2);
            "#,
        ingredient.singular_name,
        ingredient.plural_name
    )
    .execute(&app_state.pool)
    .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(sqlx::Error::Database(_)) => StatusCode::CONFLICT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn remove_ingredient(
    State(app_state): State<AppState>,
    Json(delete_ingredient_request): Json<RemoveIngredientRequest>,
) -> StatusCode {
    match sqlx::query!(
        "DELETE FROM ingredient WHERE ingredient_id = $1",
        delete_ingredient_request.ingredient_id
    )
    .execute(&app_state.pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
pub async fn update_ingredient(
    State(app_state): State<AppState>,
    Path(ingredient_id): Path<i32>,
    Json(ingredient): Json<Ingredient>,
) -> StatusCode {
    match sqlx::query!(
        "UPDATE ingredient SET singular_name = $1, plural_name = $2 WHERE ingredient_id = $3",
        ingredient.singular_name,
        ingredient.plural_name,
        ingredient_id,
    )
    .execute(&app_state.pool)
    .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::NO_CONTENT
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
