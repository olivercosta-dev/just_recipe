use axum::extract::Query;
use sqlx::PgPool;

use crate::{application::error::AppError, utilities::queries::PaginationQuery};

use super::Unit;

pub async fn fetch_units_from_db(
    query: &Query<PaginationQuery>,
    pool: &PgPool,
) -> Result<Vec<Unit>, AppError> {
    let result = sqlx::query_as!(
        Unit,
        r#" SELECT * 
            FROM unit
            WHERE unit_id >= $1
            ORDER BY unit_id
            LIMIT $2
        "#,
        query.start_from,
        query.limit + 1,
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn fetch_unit_from_db(pool: &PgPool, unit_id: i32) -> Result<Unit, AppError> {
    let unit = sqlx::query_as!(
        Unit,
        r#"
            SELECT * 
            FROM unit
            WHERE unit_id = $1
        "#,
        unit_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;
    Ok(unit)
}