use std::default;

use axum::{body::to_bytes, http::StatusCode};
use fake::Fake;
use just_recipe::{
    application::{app::App, state::AppState},
    ingredient::Ingredient,
    routes::GetIngredientsResponse,
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use crate::{choose_random_ingredient, create_get_request_to};
#[sqlx::test(fixtures(path = "../fixtures", scripts("ingredients")))]
async fn getting_existing_ingredient_returns_ingredient_and_200_ok(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let ingredient = choose_random_ingredient(&app_state.pool).await;
    let json = json!({}); // this is not needed for a get
    let request = create_get_request_to(
        "ingredients",
        Some(ingredient.ingredient_id.unwrap()),
        None,
        json,
    );
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body bytes");

    let response_ingredient: Ingredient =
        serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

    assert_eq!(response_ingredient.ingredient_id, ingredient.ingredient_id);
    assert_eq!(response_ingredient.singular_name, ingredient.singular_name);
    assert_eq!(response_ingredient.plural_name, ingredient.plural_name);
    Ok(())
}
#[sqlx::test(fixtures(path = "../fixtures", scripts("ingredients")))]
async fn getting_non_existent_ingredient_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let json = json!({}); // won't be needing this
    let request = create_get_request_to("ingredients", Some(-1), None, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
// The desired behavior:
// 1.) Sending a GET request to /ingredients returns
//     a.) List of n (0 <= n <= 15) ingredients (sorted by id)
//     b.) The next id to query from
//     c.) 200 OK
//     d.) Query Params: limit and start_from
// 2.) The response JSON Format,
//     for request at /ingredients,
//     limit is mandatory.
// {
//   ingredients: [
//      {
//        ingredient_id: 1,
//        singular_name: "Apple"
//        plural_name: "Apples"
//      },
//      ...
//   ]
//   next: "start_from=5&limit=15"
// }
// TODO (oliver) Make all the sad paths!
#[sqlx::test(fixtures(path = "../fixtures", scripts("ingredients")))]
async fn getting_ingredients_returns_ingredients_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let limit: i64 = (1..=15).fake();
    let mut start_from: Option<i32> = None;
    loop {
        let mut query_string = format!("limit={}", limit);
        if let Some(start_id) = start_from {
            query_string = format!("{}&start_from={}", query_string, start_id);
        }
        let query_params = Some(query_string);
        let json = json!({});
        let request = create_get_request_to("ingredients", None, query_params, json);
        let response = app.router.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read body bytes");

        let response_ingredients: GetIngredientsResponse =
            serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");
     
        assert!(response_ingredients.ingredients.len() as i64 <= limit);
        let start_id = if let Some(start_id) = start_from {
            start_id
        } else {
            -1
        };
        let ingredients_in_db = sqlx::query_as!(
            Ingredient,
            r#" 
                SELECT * 
                FROM ingredient
                WHERE ingredient_id >= $1
                ORDER BY ingredient_id
                LIMIT $2;
            "#,
            start_id,
            limit
        )
        .fetch_all(&app_state.pool)
        .await
        .unwrap();
        assert_eq!(
            response_ingredients.ingredients.len(),
            ingredients_in_db.len(),
        );

        assert_ingredients_match(&response_ingredients.ingredients, &ingredients_in_db);
        if response_ingredients.next_start_from.is_none() {
            break;
        } else {
            start_from = response_ingredients.next_start_from;
        }
    }

    Ok(())
}

fn assert_ingredients_match(left_ingredients: &[Ingredient], right_ingredients: &[Ingredient]) {
    // Ensure both ingredient slices are sorted by ingredient_id
    let mut left_sorted = left_ingredients.to_vec();
    let mut right_sorted = right_ingredients.to_vec();

    left_sorted.sort_by_key(|ingredient| ingredient.ingredient_id);
    right_sorted.sort_by_key(|ingredient| ingredient.ingredient_id);

    assert_eq!(
        left_sorted.len(),
        right_sorted.len(),
        "The number of ingredients does not match."
    );

    for (left, right) in left_sorted.iter().zip(right_sorted.iter()) {
        assert_eq!(
            left.ingredient_id, right.ingredient_id,
            "Ingredient ID mismatch: left = {:?}, right = {:?}",
            left, right
        );
        assert_eq!(
            left.singular_name, right.singular_name,
            "Singular name mismatch: left = {:?}, right = {:?}",
            left, right
        );
        assert_eq!(
            left.plural_name, right.plural_name,
            "Plural name mismatch: left = {:?}, right = {:?}",
            left, right
        );
    }
}
