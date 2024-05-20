use fake::Fake;
use sqlx::PgPool;

use crate::unit::Unit;

/// Chooses a random unit from the database.
///
/// This function queries the database to fetch all units and selects one at random.
/// If no units are found, it will panic with the message "No units were found.".
///
/// # Parameters
/// - `pool`: A reference to the PostgreSQL connection pool (`PgPool`).
///
/// # Returns
/// - `Unit`: The randomly chosen `Unit` instance.
///
/// # Panics
/// This function will panic if:
/// - The query to fetch units from the database fails.
/// - No units are found in the database.
pub async fn choose_random_unit(pool: &PgPool) -> Unit {
    let units = sqlx::query_as!(Unit, "SELECT * from unit")
        .fetch_all(pool)
        .await
        .expect("No units were found.");
    let random_index = (0..units.len()).fake::<usize>();
    Unit {
        unit_id: units[random_index].unit_id,
        singular_name: units[random_index].singular_name.clone(),
        plural_name: units[random_index].plural_name.clone(),
    }
}
