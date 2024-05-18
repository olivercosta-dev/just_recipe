use std::marker::PhantomData;

use crate::application::error::{AppError, RecipeParsingError};
use dashmap::DashSet;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::{
    recipe_ingredient::{CompactRecipeIngredient, DetailedRecipeIngredient, RecipeIngredient},
    recipe_step::RecipeStep,
};

// TODO (oliver): Make the recipe step always sorted!
// Backed means each unit, and ingredient is backed by the database,
// It does not mean that the recipe necessarily exists!
#[derive(Deserialize, Serialize, Debug)]
pub struct Recipe<I: RecipeIngredient, BackedState = NotBacked> {
    recipe_id: Option<i32>,
    name: String,
    description: String,
    ingredients: Vec<I>,
    steps: Vec<RecipeStep>,
    #[serde(skip)]
    backed_state: PhantomData<BackedState>,
}
pub struct Backed;
pub struct NotBacked;

// General implementations for ALL recipe states/types.
impl<I: RecipeIngredient, BackedState> Recipe<I, BackedState> {
    pub fn recipe_id(&self) -> Option<&i32> {
        self.recipe_id.as_ref()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn ingredients(&self) -> &[I] {
        &self.ingredients
    }
    pub fn steps(&self) -> &[RecipeStep] {
        &self.steps
    }
}

// Specific implementations for Detailed & Backed recipes.
impl Recipe<DetailedRecipeIngredient, Backed> {
    pub fn new(
        recipe_id: i32,
        name: String,
        description: String,
        ingredients: Vec<DetailedRecipeIngredient>,
        steps: Vec<RecipeStep>,
    ) -> Recipe<DetailedRecipeIngredient, Backed> {
        Recipe {
            recipe_id: Some(recipe_id),
            name,
            description,
            ingredients,
            steps,
            backed_state: PhantomData,
        }
    }
}

// Specific implementations for CompactRecipes, with any state.
impl<BackedState> Recipe<CompactRecipeIngredient, BackedState> {
    pub fn validate(self) -> Result<Self, AppError> {
        // Recipe_id should always be non-negative
        // If recipe_id isn't set, that means it should not even be taken into account.
        // So it will just skip this check when setting it to 0.
        if self.recipe_id.unwrap_or(0) < 0 {
            return Err(AppError::RecipeParsingError(
                RecipeParsingError::RecipeIdNotPositive,
            ));
        }
        let mut ordered_recipe_steps = self.steps.clone();
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
        Ok(self)
    }
}

// Specific implemenations for Compact & Not Backed
impl Recipe<CompactRecipeIngredient, NotBacked> {
    pub fn to_backed(
        self,
        unit_ids: &DashSet<i32>,
        ingredient_ids: &DashSet<i32>,
    ) -> Result<Recipe<CompactRecipeIngredient, Backed>, AppError> {
        let contains_invalid_ingredient_id = self
            .ingredients
            .iter()
            .find(|recipe_ingredient| !ingredient_ids.contains(&recipe_ingredient.ingredient_id))
            .is_some();
        if contains_invalid_ingredient_id {
            return Err(AppError::RecipeParsingError(
                RecipeParsingError::InvalidIngredientId,
            ));
        }
        let contains_invalid_unit_id = self
            .ingredients
            .iter()
            .find(|recipe_ingredient| !unit_ids.contains(&recipe_ingredient.unit_id))
            .is_some();
        if contains_invalid_unit_id {
            return Err(AppError::RecipeParsingError(
                RecipeParsingError::InvalidUnitId,
            ));
        }
        Ok(Recipe {
            recipe_id: self.recipe_id,
            name: self.name,
            description: self.description,
            ingredients: self.ingredients,
            steps: self.steps,
            backed_state: PhantomData,
        })
    }
}

impl Recipe<DetailedRecipeIngredient, NotBacked> {
    pub async fn to_backed(
        self,
        pool: &PgPool,
    ) -> Result<Recipe<DetailedRecipeIngredient, Backed>, AppError> {
        // OPTIMIZE (oliver): This is very expensive, in terms of DB queries
        for ingredient in self.ingredients.iter() {
            if ingredient.ingredient.ingredient_id.is_none() {
                return Err(AppError::RecipeParsingError(
                    RecipeParsingError::InvalidIngredientId,
                ));
            }
            let _ = sqlx::query!(
                r#"
                    SELECT *
                    FROM ingredient
                    WHERE ingredient_id = $1
                    AND singular_name = $2
                    AND plural_name = $3
                "#,
                ingredient.ingredient.ingredient_id.unwrap(),
                ingredient.ingredient.singular_name,
                ingredient.ingredient.plural_name,
            )
            .fetch_optional(pool)
            .await?
            .ok_or(AppError::RecipeParsingError(
                RecipeParsingError::InvalidIngredientId,
            ))?;

            if ingredient.unit.unit_id.is_none() {
                return Err(AppError::RecipeParsingError(
                    RecipeParsingError::InvalidUnitId,
                ));
            }
            let _ = sqlx::query!(
                r#"
                    SELECT *
                    FROM unit
                    WHERE unit_id = $1
                    AND singular_name = $2
                    AND plural_name = $3
                "#,
                ingredient.unit.unit_id.unwrap(),
                ingredient.unit.singular_name,
                ingredient.unit.plural_name,
            )
            .fetch_optional(pool)
            .await?
            .ok_or(AppError::RecipeParsingError(
                RecipeParsingError::InvalidUnitId,
            ))?;
        }
        Ok(Recipe {
            recipe_id: self.recipe_id,
            name: self.name,
            description: self.description,
            ingredients: self.ingredients,
            steps: self.steps,
            backed_state: PhantomData,
        })
    }
}
