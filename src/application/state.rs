use dashmap::DashSet;
use sqlx::PgPool;

// OPTIMIZE (oliver): Encapsulation could be better here.
//  Attributes shouldn't be public!
#[derive(Clone)]
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
