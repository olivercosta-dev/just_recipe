use fake::Fake;
use sqlx::PgPool;

use crate::ingredient::Ingredient;

/// Chooses a random ingredient from the database.
///
/// This function queries the database to fetch all ingredients and selects one at random.
/// If no ingredients are found, it will panic with the message "No ingredients were found.".
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `Ingredient`: The randomly chosen `Ingredient` instance.
///
/// # Panics
/// This function will panic if:
/// - The query to fetch ingredients from the database fails.
/// - No ingredients are found in the database.
pub async fn choose_random_ingredient(pool: &PgPool) -> Ingredient {
    let ingredients = sqlx::query_as!(Ingredient, "SELECT * from ingredient")
        .fetch_all(pool)
        .await
        .expect("No ingredients were found.");
    let random_index = (0..ingredients.len()).fake::<usize>();
    Ingredient {
        ingredient_id: ingredients[random_index].ingredient_id,
        singular_name: ingredients[random_index].singular_name.clone(),
        plural_name: ingredients[random_index].plural_name.clone(),
    }
}
