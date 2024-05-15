use core::marker::PhantomData;
use dashmap::DashSet;
use serde::{Deserialize, Serialize};

use sqlx::PgPool;

use crate::{app::AppError, ingredient::Ingredient, unit::Unit};

// TODO (oliver): Make the recipe step always sorted!
// Backed means each unit, and ingredient is backed by the database
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
pub struct Backed;
pub struct NotBacked;

// The point of RecipeIngredient is that a recipe ingredient
// Is (almost) always associated with a concrete recipe.
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
    recipe_id: i32,
    unit_id: i32,
    ingredient_id: i32,
    quantity: String,
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

#[derive(Serialize, Deserialize)]
pub struct DetailedRecipeIngredient {
    #[serde(skip)]
    recipe_id: i32,
    ingredient: Ingredient,
    unit: Unit,
    quantity: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecipeStep {
    #[serde(skip)]
    pub step_id: i32,
    #[serde(skip)]
    pub recipe_id: i32,
    pub step_number: i32,
    pub instruction: String,
}

#[derive(Debug, PartialEq)]
pub enum RecipeParsingError {
    StepNumbersOutOfOrder,
    RecipeIdNotPositive,
    InvalidUnitId,
    InvalidIngredientId,
    DuplicateIngredientId,
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
            let res = sqlx::query!(
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
            let res = sqlx::query!(
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

impl From<RecipeParsingError> for AppError {
    fn from(err: RecipeParsingError) -> Self {
        AppError::RecipeParsingError(err)
    }
}

// TODO (oliver): Write these tests!
// #[cfg(test)]
// mod tests {
//     use crate::{
//         app::AppError,
//         recipe::{ RecipeParsingError, RecipeStep},
//     };
//     // TODO (oliver): There is some refactoring here to be done. Make it cleaner, more general.
//     #[test]
//     fn parsing_unchecked_recipe_with_non_existent_unit_id_returns_error() {
//         use dashmap::DashSet;
//         use fake::{Fake, Faker};
//         let recipe_id = (1..100).fake::<i32>();

//         let unit_ids = DashSet::new();
//         unit_ids.insert(1);

//         let ingredient_ids = DashSet::new();
//         ingredient_ids.insert(1);

//         let invalid_unit_id = 100_000;

//         let recipe_ingredients = vec![CompressedIngredient {
//             recipe_id: recipe_id,
//             unit_id: invalid_unit_id,
//             ingredient_id: 1,
//             quantity: Faker.fake::<String>(),
//         }];
//         let steps = vec![RecipeStep {
//             step_id: 0,
//             recipe_id: recipe_id,
//             instruction: String::from("Step 1"),
//             step_number: 1,
//         }];
//         let unchecked_recipe = CompactRecipe {
//             recipe_id,
//             name: Faker.fake::<String>(),
//             description: Faker.fake::<String>(),
//             ingredients: recipe_ingredients,
//             steps,
//         };
//         let error = CompactRecipe::parse(unchecked_recipe, &unit_ids, &ingredient_ids)
//             .expect_err("should have been an error");
//         assert_eq!(
//             error,
//             AppError::RecipeParsingError(RecipeParsingError::InvalidUnitId)
//         )
//     }
//     #[test]
//     fn parsing_unchecked_recipe_with_non_existent_ingredient_id_returns_error() {
//         use dashmap::DashSet;
//         use fake::{Fake, Faker};
//         let recipe_id = (1..100).fake::<i32>();

//         let unit_ids = DashSet::new();
//         unit_ids.insert(1);

//         let ingredient_ids = DashSet::new();
//         ingredient_ids.insert(1);

//         let invalid_ingredient_id = 100_000;

//         let recipe_ingredients = vec![CompressedIngredient {
//             recipe_id: recipe_id,
//             unit_id: 1,
//             ingredient_id: invalid_ingredient_id,
//             quantity: Faker.fake::<String>(),
//         }];
//         let steps = vec![RecipeStep {
//             step_id: 0,
//             recipe_id: recipe_id,
//             instruction: String::from("Step 1"),
//             step_number: 1,
//         }];
//         let unchecked_recipe = CompactRecipe {
//             recipe_id,
//             name: Faker.fake::<String>(),
//             description: Faker.fake::<String>(),
//             ingredients: recipe_ingredients,
//             steps,
//         };
//         let error = CompactRecipe::parse(unchecked_recipe, &unit_ids, &ingredient_ids)
//             .expect_err("should have been an error");
//         assert_eq!(
//             error,
//             AppError::RecipeParsingError(RecipeParsingError::InvalidIngredientId)
//         )
//     }
// }
