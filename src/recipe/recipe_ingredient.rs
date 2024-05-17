use serde::{Deserialize, Serialize};

use crate::{ingredient::Ingredient, unit::Unit};

// The point of RecipeIngredient is that a RecipeIngredient
// Is (almost) always associated with a concrete (existing) recipe.
// It is the glue between a recipe, an ingredient, a unit, and a quantity
pub trait RecipeIngredient {
    type IngredientType;
    type UnitType;
    fn ingredient(&self) -> &Self::IngredientType;
    fn unit(&self) -> &Self::UnitType;
    fn quantity(&self) -> &str;
}

#[derive(Serialize, Deserialize)]
pub struct CompactRecipeIngredient {
    #[serde(skip)]
    pub(crate) recipe_id: i32,
    pub(crate) unit_id: i32,
    pub(crate) ingredient_id: i32,
    pub(crate) quantity: String,
}
#[derive(Serialize, Deserialize)]
pub struct DetailedRecipeIngredient {
    #[serde(skip)]
    pub(crate) recipe_id: i32,
    pub(crate) ingredient: Ingredient,
    pub(crate) unit: Unit,
    pub(crate) quantity: String,
}

impl CompactRecipeIngredient {
    pub fn new(recipe_id: i32, unit_id: i32, ingredient_id: i32, quantity: String) -> Self {
        CompactRecipeIngredient {
            recipe_id,
            unit_id,
            ingredient_id,
            quantity,
        }
    }
}

impl DetailedRecipeIngredient {
    pub fn new(recipe_id: i32, ingredient: Ingredient, unit: Unit, quantity: String) -> Self {
        DetailedRecipeIngredient {
            recipe_id,
            unit,
            ingredient,
            quantity,
        }
    }
}

impl RecipeIngredient for CompactRecipeIngredient {
    type IngredientType = i32;
    type UnitType = i32;
    fn ingredient(&self) -> &Self::IngredientType {
        &self.ingredient_id
    }

    fn unit(&self) -> &Self::UnitType {
        &self.unit_id
    }

    fn quantity(&self) -> &str {
        &self.quantity
    }
}

impl RecipeIngredient for DetailedRecipeIngredient {
    type IngredientType = Ingredient;
    type UnitType = Unit;
    fn ingredient(&self) -> &Self::IngredientType {
        &self.ingredient
    }

    fn unit(&self) -> &Self::UnitType {
        &self.unit
    }

    fn quantity(&self) -> &str {
        &self.quantity
    }
}
