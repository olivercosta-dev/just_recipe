use crate::app::AppError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename(deserialize = "recipe"))]
pub struct UncheckedRecipe {
    #[serde(skip)]
    pub recipe_id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Vec<RecipeStep>,
}

#[derive(Serialize)]
pub struct Recipe {
    pub recipe_id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Vec<RecipeStep>,
}

#[derive(Serialize, Deserialize)]
pub struct RecipeIngredient {
    #[serde(skip)]
    pub _recipe_id: i32, // shouldn't really be used outside of the Recipe
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub quantity: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RecipeStep {
    #[serde(skip)]
    pub _step_id: i32,
    #[serde(skip)]
    pub recipe_id: i32,
    pub step_number: i32,
    pub instruction: String,
}

#[derive(Debug)]
pub enum RecipeParsingError {
    StepNumbersOutOfOrder,
    RecipeIdNotPositive,
    InvalidUnitId,
    InvalidIngredientId,
    DuplicateIngredientId,
}

impl TryFrom<UncheckedRecipe> for Recipe {
    type Error = AppError;

    fn try_from(unchecked_recipe: UncheckedRecipe) -> Result<Self, AppError> {
        // Recipe_id should always be non-negative
        if unchecked_recipe.recipe_id < 0 {
            return Err(AppError::RecipeParsingError(
                RecipeParsingError::RecipeIdNotPositive,
            ));
        }
        let mut ordered_recipe_steps = unchecked_recipe.steps.clone();
        ordered_recipe_steps.sort_by(|a, b| a.step_number.cmp(&b.step_number));

        // Only recipes with complete steps (no holes, and in-order) are allowed.
        if ordered_recipe_steps[0].step_number != 1 {
            return Err(AppError::RecipeParsingError(
                RecipeParsingError::StepNumbersOutOfOrder,
            ));
        }
        // Only recipes with steps in correct order are allowed.
        for index in 0..ordered_recipe_steps.len() - 1 {
            if ordered_recipe_steps[index].step_number
                >= ordered_recipe_steps[index + 1].step_number
                || ordered_recipe_steps[index].step_number + 1
                    != ordered_recipe_steps[index + 1].step_number
            {
                return Err(AppError::RecipeParsingError(
                    RecipeParsingError::StepNumbersOutOfOrder,
                ));
            }
        }
        // TODO (oliver): Does unit_id exist? Does ingredient_id exist?
        Ok(Recipe {
            recipe_id: unchecked_recipe.recipe_id,
            name: unchecked_recipe.name,
            description: unchecked_recipe.description,
            ingredients: unchecked_recipe.ingredients,
            steps: unchecked_recipe.steps,
        })
    }
}

impl From<RecipeParsingError> for AppError {
    fn from(err: RecipeParsingError) -> Self {
        AppError::RecipeParsingError(err)
    }
}
