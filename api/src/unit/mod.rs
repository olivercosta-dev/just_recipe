pub mod helpers;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Unit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<i32>,
    pub singular_name: String,
    pub plural_name: String,
}
