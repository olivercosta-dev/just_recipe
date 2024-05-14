use std::ops::Not;

use crate::{app::AppError, ingredient::Ingredient, unit::Unit};
use core::marker::PhantomData;
use dashmap::DashSet;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

// TODO (oliver): Make the recipe step always sorted!

#[derive(Serialize, Deserialize)]
struct Recipe<I: RecipeIngredient, BackedState> {
    recipe_id: Option<i32>,
    name: String,
    ingredients: I,
    steps: Vec<RecipeStep>,
    #[serde(skip)]
    backed_state: PhantomData<BackedState>,
}

struct Backed;
struct NotBacked;

pub trait RecipeIngredient {
    type Ingredient;
    type Unit;
    fn ingredient(&self) -> &Ingredient;
    fn unit(&self) -> &Unit;
    fn quantity(&self) -> &str;
}

#[derive(Serialize, Deserialize)]
struct CompactIngredient {
    unit_id: i32,
    ingredient_id: i32,
    quantity: String,
}

#[derive(Serialize, Deserialize)]
struct DetailedIngredient {
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

impl RecipeIngredient for CompactIngredient {
    type Ingredient = i32;
    type Unit = i32;

    fn ingredient(&self) -> i32 {
        self.ingredient_id
    }

    fn unit(&self) -> i32 {
        self.unit_id
    }

    fn quantity(&self) -> &str {
        &self.quantity
    }
}

impl RecipeIngredient for DetailedIngredient {
    type Ingredient = Ingredient;
    type Unit = Unit;
    fn ingredient(&self) -> Ingredient {
        self.ingredient
    }

    fn unit(&self) -> Unit {
        self.unit
    }

    fn quantity(&self) -> &str {
        &self.quantity
    }
}

// pub async fn parse_detailed(recipe: CompactRecipe, pool: &PgPool) -> Result<DbRecipe, AppError> {
//     let ingr_ids = recipe
//         .ingredients
//         .iter()
//         .map(|ingr| ingr.ingredient_id)
//         .collect_vec();
//     let ingredients: Vec<Ingredient> = sqlx::query_as!(
//         Ingredient,
//         r#"
//                 SELECT *
//                 FROM ingredient
//                 WHERE ingredient_id = ANY($1)
//             "#,
//         &ingr_ids
//     )
//     .fetch_all(pool)
//     .await?;
//     Ok(DbRecipe {
//         recipe_id: recipe.recipe_id,
//         name: recipe.name,
//         description: recipe.description,
//         ingredients,
//         steps: recipe.steps,
//     })
// }

// impl TryFrom<CompactRecipe> for CompactRecipe {
//     type Error = AppError;
//     fn try_from(unchecked_recipe: CompactRecipe) -> Result<Self, AppError> {
//         // Recipe_id should always be non-negative
//         if unchecked_recipe.recipe_id < 0 {
//             return Err(AppError::RecipeParsingError(
//                 RecipeParsingError::RecipeIdNotPositive,
//             ));
//         }
//         let mut ordered_recipe_steps = unchecked_recipe.steps.clone();
//         ordered_recipe_steps.sort_by(|a, b| a.step_number.cmp(&b.step_number));
//         // Only recipes with complete steps (no holes, and in-order) are allowed.
//         if ordered_recipe_steps[0].step_number != 1 {
//             return Err(AppError::RecipeParsingError(
//                 RecipeParsingError::StepNumbersOutOfOrder,
//             ));
//         }
//         // Only recipes with steps in correct order are allowed.
//         for index in 0..ordered_recipe_steps.len() - 1 {
//             if ordered_recipe_steps[index].step_number
//                 >= ordered_recipe_steps[index + 1].step_number
//                 || ordered_recipe_steps[index].step_number + 1
//                     != ordered_recipe_steps[index + 1].step_number
//             {
//                 return Err(AppError::RecipeParsingError(
//                     RecipeParsingError::StepNumbersOutOfOrder,
//                 ));
//             }
//         }
//         Ok(CompactRecipe {
//             recipe_id: unchecked_recipe.recipe_id,
//             name: unchecked_recipe.name,
//             description: unchecked_recipe.description,
//             ingredients: unchecked_recipe.ingredients,
//             steps: unchecked_recipe.steps,
//         })
//     }
// }

impl Recipe<CompactIngredient, NotBacked> {
    pub fn to_backed(
        self,
        unit_ids: &DashSet<i32>,
        ingredient_ids: &DashSet<i32>,
    ) -> Result<Recipe<CompactIngredient, Backed>, AppError> {


        let recipe: CompactRecipe = unchecked_recipe.try_into()?;
        let contains_invalid_ingredient_id = recipe
            .ingredients
            .iter()
            .find(|recipe_ingredient| !ingredient_ids.contains(&recipe_ingredient.ingredient_id))
            .is_some();
        if contains_invalid_ingredient_id {
            return Err(AppError::RecipeParsingError(
                RecipeParsingError::InvalidIngredientId,
            ));
        }
        let contains_invalid_unit_id = recipe
            .ingredients
            .iter()
            .find(|recipe_ingredient| !unit_ids.contains(&recipe_ingredient.unit_id))
            .is_some();
        if contains_invalid_unit_id {
            return Err(AppError::RecipeParsingError(
                RecipeParsingError::InvalidUnitId,
            ));
        }
        Ok(recipe)
    }
}

impl From<RecipeParsingError> for AppError {
    fn from(err: RecipeParsingError) -> Self {
        AppError::RecipeParsingError(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        app::AppError,
        recipe::{
            CompactRecipe, CompactRecipe, CompressedIngredient, RecipeParsingError, RecipeStep,
        },
    };
    // TODO (oliver): There is some refactoring here to be done. Make it cleaner, more general.
    #[test]
    fn parsing_unchecked_recipe_with_non_existent_unit_id_returns_error() {
        use dashmap::DashSet;
        use fake::{Fake, Faker};
        let recipe_id = (1..100).fake::<i32>();

        let unit_ids = DashSet::new();
        unit_ids.insert(1);

        let ingredient_ids = DashSet::new();
        ingredient_ids.insert(1);

        let invalid_unit_id = 100_000;

        let recipe_ingredients = vec![CompressedIngredient {
            recipe_id: recipe_id,
            unit_id: invalid_unit_id,
            ingredient_id: 1,
            quantity: Faker.fake::<String>(),
        }];
        let steps = vec![RecipeStep {
            step_id: 0,
            recipe_id: recipe_id,
            instruction: String::from("Step 1"),
            step_number: 1,
        }];
        let unchecked_recipe = CompactRecipe {
            recipe_id,
            name: Faker.fake::<String>(),
            description: Faker.fake::<String>(),
            ingredients: recipe_ingredients,
            steps,
        };
        let error = CompactRecipe::parse(unchecked_recipe, &unit_ids, &ingredient_ids)
            .expect_err("should have been an error");
        assert_eq!(
            error,
            AppError::RecipeParsingError(RecipeParsingError::InvalidUnitId)
        )
    }
    #[test]
    fn parsing_unchecked_recipe_with_non_existent_ingredient_id_returns_error() {
        use dashmap::DashSet;
        use fake::{Fake, Faker};
        let recipe_id = (1..100).fake::<i32>();

        let unit_ids = DashSet::new();
        unit_ids.insert(1);

        let ingredient_ids = DashSet::new();
        ingredient_ids.insert(1);

        let invalid_ingredient_id = 100_000;

        let recipe_ingredients = vec![CompressedIngredient {
            recipe_id: recipe_id,
            unit_id: 1,
            ingredient_id: invalid_ingredient_id,
            quantity: Faker.fake::<String>(),
        }];
        let steps = vec![RecipeStep {
            step_id: 0,
            recipe_id: recipe_id,
            instruction: String::from("Step 1"),
            step_number: 1,
        }];
        let unchecked_recipe = CompactRecipe {
            recipe_id,
            name: Faker.fake::<String>(),
            description: Faker.fake::<String>(),
            ingredients: recipe_ingredients,
            steps,
        };
        let error = CompactRecipe::parse(unchecked_recipe, &unit_ids, &ingredient_ids)
            .expect_err("should have been an error");
        assert_eq!(
            error,
            AppError::RecipeParsingError(RecipeParsingError::InvalidIngredientId)
        )
    }
}
