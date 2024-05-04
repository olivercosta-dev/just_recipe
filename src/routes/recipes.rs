use crate::app::*;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

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
#[derive(Serialize, Deserialize)]
pub struct Recipe {
    #[serde(skip)]
    pub recipe_id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Vec<RecipeStep>,
}
#[derive(Deserialize)]
pub struct RemoveRecipeRequest {
    pub recipe_id: i32,
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
pub async fn add_recipe(
    State(app_state): State<AppState>,
    Json(recipe): Json<Recipe>,
) -> StatusCode {
    if !is_valid_recipe(&recipe) {
        return StatusCode::UNPROCESSABLE_ENTITY;
    }

    let mut transaction = match app_state.pool.begin().await {
        Ok(tr) => tr,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let recipe_query_result = match sqlx::query!(
        r#"
                INSERT INTO recipe (name, description) VALUES ($1, $2) RETURNING recipe_id
            "#,
        recipe.name,
        recipe.description
    )
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(val) => val,
        _ => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    if let Err(sqlx::Error::Database(err)) = bulk_insert_ingredients(
        recipe.ingredients,
        recipe_query_result.recipe_id,
        &mut transaction,
    )
    .await
    {
        let err_kind = err.kind();
        match err_kind {
            sqlx::error::ErrorKind::Other => return StatusCode::INTERNAL_SERVER_ERROR,
            _ => return StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
    if let Err(sqlx::Error::Database(err)) = bulk_insert_steps(
        recipe.steps,
        recipe_query_result.recipe_id,
        &mut transaction,
    )
    .await
    {
        let err_kind = err.kind();
        match err_kind {
            sqlx::error::ErrorKind::Other => return StatusCode::INTERNAL_SERVER_ERROR,
            _ => return StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
    match transaction.commit().await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn is_valid_recipe(recipe: &Recipe) -> bool {
    let mut ordered_recipe_steps = recipe.steps.clone();
    ordered_recipe_steps.sort_by(|a, b| a.step_number.cmp(&b.step_number));

    // Only recipes with complete steps (no holes, and in-order) are allowed.
    if ordered_recipe_steps[0].step_number != 1 {
        return false;
    }
    // Only recipes with steps in correct order are allowed.
    for index in 0..ordered_recipe_steps.len() - 1 {
        if ordered_recipe_steps[index].step_number >= ordered_recipe_steps[index + 1].step_number
            || ordered_recipe_steps[index].step_number + 1
                != ordered_recipe_steps[index + 1].step_number
        {
            return false;
        }
    }
    true
}

// TODO (oliver): Perhaps unit test these.
async fn bulk_insert_ingredients(
    ingredients: Vec<RecipeIngredient>,
    recipe_id: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> sqlx::Result<()> {
    let ingr_ids: Vec<i32> = ingredients.iter().map(|ingr| ingr.ingredient_id).collect();
    let unit_ids: Vec<i32> = ingredients.iter().map(|ingr| ingr.unit_id).collect();
    let quants: Vec<String> = ingredients
        .iter()
        .map(|ingr| ingr.quantity.clone())
        .collect();
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

async fn bulk_insert_steps(
    steps: Vec<RecipeStep>,
    recipe_id: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
    let step_numbers: Vec<i32> = steps.iter().map(|step| step.step_number).collect();
    let instructions: Vec<String> = steps.iter().map(|step| step.instruction.clone()).collect();
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

pub async fn remove_recipe(
    State(app_state): State<AppState>,
    Json(remove_recipe_request): Json<RemoveRecipeRequest>,
) -> StatusCode {
    match sqlx::query!(
        "DELETE FROM recipe WHERE recipe_id = $1",
        remove_recipe_request.recipe_id
    )
    .execute(&app_state.pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_)=> StatusCode::INTERNAL_SERVER_ERROR,
    }
}
