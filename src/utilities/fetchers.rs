use dashmap::DashSet;
use sqlx::PgPool;

use crate::{ingredient::Ingredient, unit::Unit};

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