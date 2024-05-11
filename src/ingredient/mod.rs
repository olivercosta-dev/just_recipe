use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Ingredient {
    #[serde(skip)]
    pub ingredient_id: i32,
    pub singular_name: String,
    pub plural_name: String,
}
