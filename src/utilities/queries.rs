use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationQuery {
    pub limit: i64,
    // Default start_id is 0
    #[serde(default)]
    pub start_from: i32,
}