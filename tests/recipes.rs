use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use fake::{Fake, Faker};
use just_recipe::app::{new_app, AppState};

use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`


#[sqlx::test(fixtures("units", "ingredients"))]
async fn adding_new_recipe_persists_and_returns_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();

    let (ingredient_id1, unit_id1, quantity1) = (1, 1, String::from("3/4"));
    let (ingredient_id2, unit_id2, quantity2) = (2, 1, String::from("2/4"));

    let (step_number1, instruction1) = (1, String::from("Boil the apple in hot water."));
    let (step_number2, instruction2) = (2, String::from("Eat the apple."));
    //TODO (oliver): Maybe the response should contain the recipe id?
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/recipes")
                .header("Content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(
                        &json!(
                            {
                                "name": recipe_name,
                                "description": description,
                                "ingredients": [ // TODO (oliver): Varying number of ingredients and steps! Should be a random number
                                    {
                                        "ingredient_id": ingredient_id1,
                                        "unit_id": unit_id1,
                                        "quantity": quantity1,
                                    },
                                    {
                                        "ingredient_id": ingredient_id2,
                                        "unit_id": unit_id2,
                                        "quantity": quantity2,
                                    }
                                ],
                                "steps": [
                                    {
                                        "step_number": step_number1,
                                        "instruction": instruction1
                                    },
                                    {
                                        "step_number": step_number2,
                                        "instruction": instruction2
                                    }
                                ]
                            }
                        ),
                    )
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .expect("Should have gotten a valid response.");

    assert_eq!(response.status(), StatusCode::OK, "Status codes should match.");

    let recipe_record = sqlx::query!(
            r#"
                SELECT recipe_id, name, description
                FROM recipe
                WHERE name = $1;
            "#,
            recipe_name,
        )
        .fetch_one(&app_state.pool)
        .await
        .expect("Should have gotten a record of a recipe.");
    
    // The description could be null/empty, but we know that in this case it definietly should not, so we can unwrap it safely!
    // if it doesn't unwrap safely there is something wrong with the other code.
    assert_eq!((recipe_record.name, recipe_record.description.unwrap()), (recipe_name, description));
    
    let recipe_ingredient_records = sqlx::query!(
            r#"
                SELECT recipe_id, ingredient_id, unit_id, quantity
                FROM recipe_ingredient
                WHERE recipe_id = $1
                ORDER BY ingredient_id;
            "#,
            recipe_record.recipe_id
        )
        .fetch_all(&app_state.pool)
        .await
        .expect("Should have gotten a result for the recipe ingredients.");
    assert_eq!(recipe_ingredient_records.len(), 2, "2 ingredient records should be returned");
    let recipe_step_records = sqlx::query!(
            r#"
                SELECT step_id, recipe_id, step_number, instruction
                FROM step
                WHERE recipe_id = $1
                ORDER BY step_number;
            "#,
            recipe_record.recipe_id
        )
        .fetch_all(&app_state.pool)
        .await
        .expect("Should have gotten a result for the recipe steps.");
    assert_eq!(recipe_step_records.len(), 2, "2 step records should be returned");
    
    
 
    assert_eq!(
        (recipe_ingredient_records[0].ingredient_id, recipe_ingredient_records[0].unit_id.unwrap(), recipe_ingredient_records[0].quantity.clone().unwrap()),
        (ingredient_id1, unit_id1, quantity1),
    );
    assert_eq!(
        (recipe_ingredient_records[1].ingredient_id, recipe_ingredient_records[1].unit_id.unwrap(), recipe_ingredient_records[1].quantity.clone().unwrap()),
        (ingredient_id2, unit_id2, quantity2)
    );
    assert_eq!(
        (recipe_step_records[0].step_number.unwrap(), recipe_step_records[0].instruction.clone().unwrap()), // fix: find an alternative to cloning
        (step_number1, instruction1)
    );
    assert_eq!(
        (recipe_step_records[1].step_number.unwrap(), recipe_step_records[1].instruction.clone().unwrap()),
        (step_number2, instruction2)
    );
  
    Ok(())
}

// #[tokio::test]
// async fn adding_recipe_with_wrong_step_numbers_errors() {

// }
// #[tokio::test]
// async fn adding_recipe_with_wrong_ingredient_id_errors() {

// }
// #[tokio::test]
// async fn adding_recipe_with_wrong_unit_id_errors() {

// }