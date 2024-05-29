use dashmap::DashSet;
use sqlx::PgPool;

// NOTE : Attributes are public, 
// NOTE : because AppState is just used in the router
// NOTE : And it's always going to be cloned anyways, as it's cheap
// NOTE : As it is an Arc<Mutex<T>>
#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub unit_ids: DashSet<i32>,
    pub ingredient_ids: DashSet<i32>,
}
impl AppState {
    pub fn new(pool: PgPool) -> Self {
        AppState {
            pool,
            unit_ids: DashSet::new(),
            ingredient_ids: DashSet::new(),
        }
    }
}
