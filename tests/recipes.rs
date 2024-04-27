use axum::{
    body::Body,
    http::{request, Request, StatusCode},
};
use fake::{Fake, Faker};
use just_recipe::{
    app::{new_app, AppState},
    utils::create_post_request_to,
};

use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

#[sqlx::test(fixtures("units", "ingredients"))]
async fn adding_new_recipe_persists_and_returns_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let (ingredient_id1, unit_id1, quantity1) = (1, 1, Faker.fake::<String>());
    let (ingredient_id2, unit_id2, quantity2) = (2, 1, Faker.fake::<String>());

    let number_of_steps = (2..10).fake::<i32>();

    let steps: Vec<Value> = (1..=number_of_steps)
        .into_iter()
        .map(|number| json!({"step_number": number, "instruction": Faker.fake::<String>()}))
        .collect();
    let json = json!(
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
            "steps": steps
        }
    );
    let request = create_post_request_to("recipes", json);
    let response = app
        .oneshot(request)
        .await
        .expect("Should have gotten a valid response.");

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status codes should match."
    );

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
    // if it doesn't unwrap safely there must be something wrong.
    assert_eq!(
        (recipe_record.name, recipe_record.description.unwrap()),
        (recipe_name, description)
    );

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
    assert_eq!(
        recipe_ingredient_records.len(),
        2,
        "2 ingredient records should be returned"
    );

    let ordered_recipe_step_records = sqlx::query!(
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
    assert_eq!(
        ordered_recipe_step_records.len(),
        steps.len(),
        "Step numbers should match"
    );

    assert_eq!(
        (
            recipe_ingredient_records[0].ingredient_id,
            recipe_ingredient_records[0].unit_id.unwrap(),
            recipe_ingredient_records[0].quantity.clone().unwrap()
        ),
        (ingredient_id1, unit_id1, quantity1),
    );
    assert_eq!(
        (
            recipe_ingredient_records[1].ingredient_id,
            recipe_ingredient_records[1].unit_id.unwrap(),
            recipe_ingredient_records[1].quantity.clone().unwrap()
        ),
        (ingredient_id2, unit_id2, quantity2)
    );

    for (index, step) in steps.iter().enumerate() {
        let step_number = i32::try_from(
            step["step_number"]
                .as_i64()
                .expect("Should have been an integer"),
        )
        .expect("Should have been an i32");
        let instruction = String::from(
            step["instruction"]
                .as_str()
                .expect("Should have been a string"),
        );
        let recipe_step_record = &ordered_recipe_step_records[index];
        let (record_step_number, record_instruction) = (
            recipe_step_record.step_number.unwrap(),
            (recipe_step_record.instruction.clone().unwrap()),
        );
        assert_eq!(
            (record_step_number, record_instruction),
            (step_number, instruction)
        );
    }

    Ok(())
}

#[sqlx::test(fixtures("units", "ingredients"))]
async fn adding_recipe_with_wrong_step_numbers_returns_422_unproccessable_entity(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let (ingredient_id1, unit_id1, quantity1) = (1, 1, String::from("3/4"));
    let (step_number1, instruction1) = (1, Faker.fake::<String>());
    let (step_number2, instruction2) = (7, Faker.fake::<String>());

    let json = json!(
        {
            "name": recipe_name,
            "description": description,
            "ingredients": [
                {
                    "ingredient_id": ingredient_id1,
                    "unit_id": unit_id1,
                    "quantity": quantity1,
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
    );
    let request = create_post_request_to("recipes", json);
    let response = app
        .oneshot(request)
        .await
        .expect("Should have gotten a valid response.");
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[sqlx::test(fixtures("units"))]
async fn adding_recipe_with_non_existent_ingredient_id_returns_422_unproccessable_entity(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let (ingredient_id, unit_id, quantity) = (Faker.fake::<i32>(), 1, String::from("3/4"));
    let (step_number, instruction) = (1, Faker.fake::<String>());
    let json = json!(
        {
            "name": recipe_name,
            "description": description,
            "ingredients": [
                {
                    "ingredient_id": ingredient_id,
                    "unit_id": unit_id,
                    "quantity": quantity,
                }
            ],
            "steps": [
                {
                    "step_number": step_number,
                    "instruction": instruction
                }
            ]
        }
    );
    let request = create_post_request_to("recipes", json);
    let response = app
        .oneshot(request)
        .await
        .expect("Should have gotten a valid response.");
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}

#[sqlx::test(fixtures("ingredients"))]
async fn adding_recipe_with_non_existent_unit_id_returns_422_unproccessable_entity(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let recipe_name = Faker.fake::<String>();
    let description = Faker.fake::<String>();
    let (ingredient_id, unit_id, quantity) = (1, Faker.fake::<i32>(), String::from("3/4"));
    let (step_number, instruction) = (1, Faker.fake::<String>());
    let json = json!(
        {
            "name": recipe_name,
            "description": description,
            "ingredients": [
                {
                    "ingredient_id": ingredient_id,
                    "unit_id": unit_id,
                    "quantity": quantity,
                }
            ],
            "steps": [
                {
                    "step_number": step_number,
                    "instruction": instruction
                }
            ]
        }
    );
    let request = create_post_request_to("recipes", json);
    let response = app
        .oneshot(request)
        .await
        .expect("Should have gotten a valid response.");
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}
