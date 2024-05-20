use axum::extract::Query;
use sqlx::PgPool;


use crate::{application::error::AppError, utilities::queries::PaginationQuery};

use super::Ingredient;


/// Fetches exactly ONE MORE ingredient than in the query!
pub async fn fetch_ingredients_from_db(
    query: &Query<PaginationQuery>,
    pool: &PgPool,
) -> Result<Vec<Ingredient>, AppError> {
    let result = sqlx::query_as!(
        Ingredient,
        r#" SELECT * 
            FROM ingredient
            WHERE ingredient_id >= $1
            ORDER BY ingredient_id
            LIMIT $2;
        "#,
        query.start_from,
        (query.limit + 1),
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}