use axum::extract::Query;
use dashmap::DashSet;
use sqlx::{Executor, PgPool, Postgres};
use tracing::instrument;

use crate::{
    application::error::AppError,
    ingredient::Ingredient,
    recipe::{
        recipe::{Backed, Recipe},
        recipe_ingredient::DetailedRecipeIngredient,
        recipe_step::RecipeStep,
    },
    unit::Unit,
};

use super::queries::PaginationQuery;

/// Fetches all unit IDs from the database and returns them as a `DashSet`.
///
/// This function queries the database for all unit IDs from the `unit` table and returns them
/// as a `DashSet<i32>`. If the query fails, it returns `None`.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `Option<DashSet<i32>>`: Returns `Some(DashSet<i32>)` containing all unit IDs if the query
///   is successful. Returns `None` if the query fails.
pub async fn fetch_all_unit_ids(pool: &PgPool) -> Option<DashSet<i32>> {
    let unit_records = sqlx::query!("SELECT unit_id FROM unit")
        .fetch_all(pool)
        .await;
    if unit_records.is_err() {
        return None;
    }
    let units: DashSet<i32> = DashSet::new();

    for record in unit_records.unwrap() {
        units.insert(record.unit_id);
    }
    Some(units)
}
/// Fetches all ingredient IDs from the database and returns them as a `DashSet`.
///
/// This function queries the database for all ingredient IDs from the `ingredient` table and returns them
/// as a `DashSet<i32>`. If the query fails, it returns `None`.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `Option<DashSet<i32>>`: Returns `Some(DashSet<i32>)` containing all ingredient IDs if the query
///   is successful. Returns `None` if the query fails.
pub async fn fetch_all_ingredient_ids(pool: &PgPool) -> Option<DashSet<i32>> {
    let ingredient_records = sqlx::query!("SELECT ingredient_id FROM ingredient")
        .fetch_all(pool)
        .await;
    if ingredient_records.is_err() {
        return None;
    }
    let ingredients: DashSet<i32> = DashSet::new();

    for record in ingredient_records.unwrap() {
        ingredients.insert(record.ingredient_id);
    }
    Some(ingredients)
}

/// Fetches all ingredients and units from the database.
///
/// This function queries the database to fetch all records from the `ingredient` and `unit` tables.
/// It returns a tuple containing two vectors: one with all the ingredients and one with all the units.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `(Vec<Ingredient>, Vec<Unit>)`: A tuple containing two vectors. The first vector contains all
///   the `Ingredient` instances, and the second vector contains all the `Unit` instances.
///
/// # Panics
/// This function will panic if:
/// - The query to fetch ingredients from the database fails.
/// - The query to fetch units from the database fails.
pub async fn fetch_ingredients_and_units(pool: &PgPool) -> (Vec<Ingredient>, Vec<Unit>) {
    let all_ingredients = sqlx::query_as!(Ingredient, "SELECT * FROM ingredient")
        .fetch_all(pool)
        .await
        .expect("Should have had at least 1 ingredient in the database");

    let all_units = sqlx::query_as!(Unit, "SELECT * FROM unit")
        .fetch_all(pool)
        .await
        .expect("Should have had at least 1 unit in the database");

    (all_ingredients, all_units)
}

/// Fetches units from the database with pagination.
///
/// This function queries the database to fetch units starting from a specified unit ID, ordered by unit ID.
/// It uses the given pagination parameters to limit the number of results returned. The result is a vector of `Unit` instances. <br>
/// <b>Note that the result will contain one more unit than the specified limit to help with pagination logic!</b>
///
/// # Parameters
/// - `query`: A reference to a `Query<PaginationQuery>` that contains the pagination parameters:
///   - `start_from`: The starting unit ID for the query.
///   - `limit`: The maximum number of units to fetch. (The result will contain maximum limit + 1 units)
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `Result<Vec<Unit>, AppError>`: A result containing a vector of `Unit` instances if the query is successful,
///   or an `AppError` if an error occurs during the query.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to fetch units from the database fails.
pub async fn fetch_units_with_pagination(
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

/// Fetches a unit from the database by its ID.
///
/// This function queries the database to fetch a unit with the specified unit ID.
/// If the unit is found, it is returned as a `Unit` instance. If the unit is not found,
/// it returns an `AppError::NotFound`.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
/// - `unit_id`: The ID of the unit to fetch.
///
/// # Returns
/// - `Result<Unit, AppError>`: A result containing the `Unit` instance if the query is successful,
///   or an `AppError::NotFound` if the unit is not found, or another `AppError` if an error occurs during the query.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to fetch the unit from the database fails.
/// - The unit with the specified ID is not found.
pub async fn fetch_unit(pool: &PgPool, unit_id: i32) -> Result<Unit, AppError> {
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

/// Fetches a recipe from the database by its ID.
///
/// This function queries the database to fetch a recipe with the specified recipe ID. It retrieves
/// the recipe details, ingredients, units, and steps associated with the recipe. If the recipe is found,
/// it is returned as a `Recipe<DetailedRecipeIngredient, Backed>` instance. If the recipe is not found,
/// it returns an `AppError::NotFound`.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
/// - `recipe_id`: The ID of the recipe to fetch.
///
/// # Returns
/// - `Result<Recipe<DetailedRecipeIngredient, Backed>, AppError>`: A result containing the `Recipe` instance if the query is successful,
///   or an `AppError::NotFound` if the recipe is not found, or another `AppError` if an error occurs during the query.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to fetch the recipe from the database fails.
/// - The recipe with the specified ID is not found.
#[instrument(ret, err)]
pub async fn fetch_recipe_detailed(
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
    let recipe = Recipe::<DetailedRecipeIngredient>::new(
        recipe_id,
        name,
        description,
        detailed_ingredients,
        steps,
    );
    Ok(recipe)
}

/// Fetches ingredients from the database with pagination.
///
/// This function queries the database to fetch ingredients starting from a specified ingredient ID, ordered by ingredient ID.
/// It uses the given pagination parameters to limit the number of results returned. The result is a vector of `Ingredient` instances. <br>
/// <b>Note that the result will contain one more unit than the specified limit to help with pagination logic!</b>
///
/// # Parameters
/// - `query`: A reference to a `Query<PaginationQuery>` that contains the pagination parameters:
///   - `start_from`: The starting ingredient ID for the query.
///   - `limit`: The maximum number of ingredients to fetch. (The result will contain maximum limit + 1 ingredients)
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `Result<Vec<Ingredient>, AppError>`: A result containing a vector of `Ingredient` instances if the query is successful,
///   or an `AppError` if an error occurs during the query.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to fetch ingredients from the database fails.
pub async fn fetch_ingredients_with_pagination(
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

/// Fetches an ingredient from the database by its ID.
///
/// This function queries the database to fetch an ingredient with the specified ingredient ID.
/// If the ingredient is found, it is returned as an `Ingredient` instance. If the ingredient is not found,
/// it returns an `AppError::NotFound`.
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
/// - `ingredient_id`: The ID of the ingredient to fetch.
///
/// # Returns
/// - `Result<Ingredient, AppError>`: A result containing the `Ingredient` instance if the query is successful,
///   or an `AppError::NotFound` if the ingredient is not found, or another `AppError` if an error occurs during the query.
///
/// # Errors
/// This function returns an `AppError` if:
/// - The query to fetch the ingredient from the database fails.
/// - The ingredient with the specified ID is not found.
pub async fn fetch_ingredient(pool: &PgPool, ingredient_id: i32) -> Result<Ingredient, AppError> {
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

pub async fn fetch_all_ingredients(
    executor: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<Ingredient>, AppError> {
    Ok(sqlx::query_as!(
            Ingredient,
            r#"
                SELECT *
                FROM ingredient
                ORDER BY singular_name;
            "#
        )
        .fetch_all(executor)
        .await?)
}

pub async fn fetch_all_units(
    executor: impl Executor<'_, Database = Postgres>,
) -> Result<Vec<Unit>, AppError> {
    Ok(sqlx::query_as!(
            Unit,
            r#"
                SELECT *
                FROM unit
                ORDER BY singular_name;
            "#
        )
        .fetch_all(executor)
        .await?)
}
