use dashmap::DashSet;
use sqlx::PgPool;

pub mod application;
pub mod ingredient;
pub mod recipe;
pub mod routes;
pub mod unit;

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
