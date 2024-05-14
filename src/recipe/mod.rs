use crate::{app::AppError, ingredient::Ingredient};
use dashmap::DashSet;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
#[serde(rename(deserialize = "recipe"))]
pub struct UncheckedRecipe {
    #[serde(skip)]
    pub recipe_id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<CompressedIngredient>,
    pub steps: Vec<RecipeStep>,
}
// TODO (oliver): Make the recipe step always sorted!
/// It is called CompressedRecipe, because 
/// it contains reduced information about the recipe.
/// (Compressed Ingredients)
#[derive(Serialize, Debug)]
pub struct CompressedRecipe {
    pub recipe_id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<CompressedIngredient>,
    pub steps: Vec<RecipeStep>,
}

#[derive(Serialize, Deserialize)]
pub struct DetailedRecipe {
    pub recipe_id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<RecipeStep>,
}
enum DbRecipe {
    Detailed(DetailedRecipe),
    Compressed(CompressedRecipe)
}
impl DbRecipe {

}
/// It is compressed ingredient, as it only contains 
/// unit_id, ingredient_id, quantity
/// without any more information. 
/// (no names for units and ingredients)
#[derive(Serialize, Deserialize, Debug)]
pub struct CompressedIngredient {
    #[serde(skip)]
    pub recipe_id: i32, // shouldn't really be used outside of the Recipe
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub quantity: String,
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

impl CompressedRecipe {
    /// Returns a fully-valid recipe, whose ingredients
    /// are backed by the database.
    pub fn parse(
        unchecked_recipe: UncheckedRecipe,
        unit_ids: &DashSet<i32>,
        ingredient_ids: &DashSet<i32>,
    ) -> Result<Self, AppError> {
        impl TryFrom<UncheckedRecipe> for CompressedRecipe {
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
                Ok(CompressedRecipe {
                    recipe_id: unchecked_recipe.recipe_id,
                    name: unchecked_recipe.name,
                    description: unchecked_recipe.description,
                    ingredients: unchecked_recipe.ingredients,
                    steps: unchecked_recipe.steps,
                })
            }
        }

        let recipe: CompressedRecipe = unchecked_recipe.try_into()?;
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
    // TODO (oliver): Make parse a trait function.
    pub async fn parse_detailed(recipe: CompressedRecipe, pool: &PgPool) -> Result<DbRecipe, AppError> {
        let ingr_ids = recipe
            .ingredients
            .iter()
            .map(|ingr| ingr.ingredient_id)
            .collect_vec();
        let ingredients: Vec<Ingredient> = sqlx::query_as!(
            Ingredient,
            r#"
                SELECT *
                FROM ingredient
                WHERE ingredient_id = ANY($1)
            "#,
            &ingr_ids
        )
        .fetch_all(pool)
        .await?;
        Ok(DbRecipe {
            recipe_id: recipe.recipe_id,
            name: recipe.name,
            description: recipe.description,
            ingredients,
            steps: recipe.steps,
        })
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
        recipe::{CompressedRecipe, CompressedIngredient, RecipeParsingError, RecipeStep, UncheckedRecipe},
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
        let unchecked_recipe = UncheckedRecipe {
            recipe_id,
            name: Faker.fake::<String>(),
            description: Faker.fake::<String>(),
            ingredients: recipe_ingredients,
            steps,
        };
        let error = CompressedRecipe::parse(unchecked_recipe, &unit_ids, &ingredient_ids)
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
        let unchecked_recipe = UncheckedRecipe {
            recipe_id,
            name: Faker.fake::<String>(),
            description: Faker.fake::<String>(),
            ingredients: recipe_ingredients,
            steps,
        };
        let error = CompressedRecipe::parse(unchecked_recipe, &unit_ids, &ingredient_ids)
            .expect_err("should have been an error");
        assert_eq!(
            error,
            AppError::RecipeParsingError(RecipeParsingError::InvalidIngredientId)
        )
    }
}
