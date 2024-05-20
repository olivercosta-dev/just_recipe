use serde::{Deserialize, Serialize};

// RecipeStep is in this folder, because it only exists in the realm 
// of a recipe, and never outside.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecipeStep {
    #[serde(skip)]
    pub step_id: i32,
    #[serde(skip)]
    pub recipe_id: i32,
    pub step_number: i32,
    pub instruction: String,
}
