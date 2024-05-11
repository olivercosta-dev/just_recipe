use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Unit {
    #[serde(skip)]
    pub unit_id: i32,
    pub singular_name: String,
    pub plural_name: String,
}
