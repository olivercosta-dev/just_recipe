use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{error::DatabaseError, postgres::PgDatabaseError};

use crate::app::*;

#[derive(Serialize, Deserialize)]
pub struct RecipeIngredient {
    #[serde(skip)]
    pub _recipe_id: i32, // shouldn't really be used outside of the Recipe
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub quantity: String
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Step {
    #[serde(skip)]
    pub _step_id: i32,
    #[serde(skip)]
    pub recipe_id: i32,
    pub step_number: i32,
    pub instruction: String,
}
#[derive(Serialize, Deserialize)]
pub struct Recipe {
    #[serde(skip)]
    pub recipe_id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Vec<Step>
}
/* Example request:
{
    "name": "Very Tasty Soup",
    "description": "Finger-licking Good!",
    "ingredients": [
        {
            "ingredient_id": 1,
            "unit_id": 1,
            "quantity": "3/4",
        },
        {
            "ingredient_id": 1,
            "unit_id": 2,
            "quantity": "1/2",
        }
    ],
    "steps": [
        {
            "step_number": 1,
            "instruction": "Put the apple in boiling hot water."
        },
        {
            "step_number": 2,
            "instruction": "Eat the apple."
        }
    ]
}
*/
//  TODO (oliver):  Better error handling. Not just panics.
// TODO (oliver):   Better error messages, with bodies and messages, not just error codes
//  TODO (oliver):  Maybe the response should contain the recipe id?
pub async fn recipes(State(app_state) : State<AppState>, Json(recipe): Json<Recipe>) -> StatusCode {
    if !is_valid_recipe(&recipe) {
        return StatusCode::UNPROCESSABLE_ENTITY
    }
  
    let mut transaction = app_state.pool.begin().await.expect("Should have began transaction");
    
    let recipe_query_result = sqlx::query!(
            r#"
                INSERT INTO recipe (name, description) VALUES ($1, $2) RETURNING recipe_id
            "#,
            recipe.name,
            recipe.description
        )
        .fetch_one(&mut *transaction)
        .await
        .expect("Should have inserted the recipe into the database");

    if let Err(sqlx::Error::Database(err)) = 
        bulk_insert_ingredients(recipe.ingredients, recipe_query_result.recipe_id, &mut transaction).await {
            if err.is_foreign_key_violation() {
                return StatusCode::UNPROCESSABLE_ENTITY;
            } else {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
    }
    if let Err(sqlx::Error::Database(err)) = 
        bulk_insert_steps(recipe.steps, recipe_query_result.recipe_id, &mut transaction).await {
            if err.is_foreign_key_violation() {
                return StatusCode::UNPROCESSABLE_ENTITY;
            } else {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
        
    transaction.commit().await.expect("Should have committed the transaction");
    StatusCode::OK
}

fn is_valid_recipe(recipe: &Recipe) -> bool{
    let mut ordered_recipe_steps = recipe.steps.clone();
    ordered_recipe_steps.sort_by(|a, b| a.step_number.cmp(&b.step_number));
    
    // Only recipes with complete steps (no holes, and in-order) are allowed.
    if ordered_recipe_steps[0].step_number != 1 {
        return false;
    }
    for index in 0..ordered_recipe_steps.len()- 1 {
        if ordered_recipe_steps[index].step_number >= ordered_recipe_steps[index + 1].step_number || ordered_recipe_steps[index].step_number + 1 != ordered_recipe_steps[index + 1].step_number{
            return false;
        }
    }
    true
}

async fn bulk_insert_ingredients(ingredients: Vec<RecipeIngredient>, recipe_id: i32, transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> sqlx::Result<()>{
    let ingr_ids: Vec<i32>= ingredients.iter().map(|ingr| ingr.ingredient_id).collect();
    let unit_ids: Vec<i32>= ingredients.iter().map(|ingr| ingr.unit_id).collect();
    let quants: Vec<String>= ingredients.iter().map(|ingr| ingr.quantity.clone()).collect();
    let rec_ids: Vec<i32> = (0..ingr_ids.len()).map(|_| recipe_id).collect();

    sqlx::query!(
            r#"
                INSERT INTO recipe_ingredient (recipe_id, ingredient_id, unit_id, quantity)
                SELECT * FROM UNNEST($1::INT[], $2::INT[], $3::INT[], $4::VARCHAR(50)[]);
            "#,
            &rec_ids,
            &ingr_ids,
            &unit_ids,
            &quants
        )
        .execute(&mut **transaction)
        .await?;
    Ok(())
}

async fn bulk_insert_steps(steps: Vec<Step>, recipe_id: i32, transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<(), sqlx::Error> {
    let step_numbers: Vec<i32>= steps.iter().map(|step| step.step_number).collect();
    let instructions: Vec<String>= steps.iter().map(|step| step.instruction.clone()).collect();
    let rec_ids: Vec<i32> = (0..step_numbers.len()).map(|_| recipe_id).collect();

    sqlx::query!(
            r#"
                INSERT INTO step (recipe_id, step_number, instruction)
                SELECT * FROM UNNEST($1::INT[], $2::INT[], $3::TEXT[]);
            "#,
            &rec_ids,
            &step_numbers,
            &instructions
        )
        .execute(&mut **transaction)
        .await?;
    Ok(())
}