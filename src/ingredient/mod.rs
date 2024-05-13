use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ingredient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredient_id: Option<i32>,
    pub singular_name: String,
    pub plural_name: String,
}
