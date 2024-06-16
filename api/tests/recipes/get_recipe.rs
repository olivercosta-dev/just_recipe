use std::{collections::HashSet, default};

use axum::{body::to_bytes, http::StatusCode};
use fake::{Fake, Faker};
use just_recipe::{
    application::{app::App, state::AppState},
    recipe::{
        recipe::Recipe,
        recipe_ingredient::{DetailedRecipeIngredient, RecipeIngredient},
    },
    routes::GetRecipesResponse,
    utilities::{
        assertions::{
            assert_detailed_ingredients_exist, assert_ingredients_exist, assert_recipe_exists,
            assert_recipe_steps_exist,
        },
        random_generation::recipes::choose_random_recipe_id,
        request_creators::create_get_request_to,
    },
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients", "steps")
))]
async fn getting_existing_recipe_returns_recipe_and_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let recipe_id = choose_random_recipe_id(&app_state.pool).await;
    let request = create_get_request_to("recipes", Some(recipe_id), None, json!({}));
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body bytes");

    let response_recipe: Recipe<DetailedRecipeIngredient> =
        serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

    let recipe_id_in_db = assert_recipe_exists(
        &app_state.pool,
        response_recipe.name(),
        response_recipe.description(),
    )
    .await;
    assert_eq!(
        response_recipe
            .recipe_id()
            .expect("recipe_id should have existed"),
        recipe_id_in_db
    );

    assert_recipe_steps_exist(&app_state.pool, response_recipe.steps(), recipe_id)
        .await
        .unwrap();
    assert_ingredients_exist(
        &app_state.pool,
        response_recipe
            .ingredients()
            .iter()
            .map(|ing| ing.ingredient())
            .collect(),
    )
    .await;
    Ok(())
}

#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients", "steps")
))]
async fn getting_non_existent_recipe_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let recipe_id = -1;
    let request = create_get_request_to("recipes", Some(recipe_id), None, json!({}));
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("units", "ingredients", "recipes", "recipe_ingredients", "steps",)
))]
async fn getting_recipes_returns_recipes_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    // let limit: i64 = (1..=15).fake(); // max 15 recipes at a time, otherwise it would be an error
    for number in 1..=15 {
        let limit = number;
        let last_response: GetRecipesResponse =
            assert_forward_pagination_works(limit, app.clone(), &app_state.pool).await;
        assert_backward_pagination_works(limit, last_response, app.clone(), &app_state.pool).await;
    }
    Ok(())
}

async fn assert_forward_pagination_works(
    limit: i64,
    app: App,
    pool: &PgPool,
) -> GetRecipesResponse {
    let mut queried_recipe_ids = HashSet::<i32>::new();
    let mut start_from: Option<i32> = None;
    loop {
        let mut query_string = format!("limit={}", limit);
        if let Some(start_id) = start_from {
            query_string = format!("{}&start_from={}", query_string, start_id);
        }
        let query_params = Some(query_string);
        let json = json!({});
        let request = create_get_request_to("recipes", None, query_params, json);
        let response = app.router.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read body bytes");

        let response_recipes: GetRecipesResponse =
            serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

        assert!(response_recipes.recipes.len() as i64 <= limit);

        // Here goes the db assertions
        for recipe in &response_recipes.recipes {
            let rec_id = recipe
                .recipe_id()
                .expect("Recipe id should have been Some(i32)");
            let newly_inserted = queried_recipe_ids.insert(recipe.recipe_id().unwrap());
            assert!(newly_inserted); // The endpoint should never give overlapping results (if the queries are consistent)
            assert_detailed_ingredients_exist(&pool, recipe.ingredients())
                .await
                .unwrap();
            assert_recipe_steps_exist(&pool, recipe.steps(), rec_id.clone())
                .await
                .unwrap();
        }
        // End of db assertions
        // After all is done, we can decide whether it is over or not
        if response_recipes.next_start_from.is_none() {
            let record = sqlx::query!(
                r#"
                    SELECT COUNT(*) as count
                    FROM recipe;
                "#
            )
            .fetch_one(pool)
            .await
            .expect("Should have had a result");
            assert_eq!(record.count.unwrap(), queried_recipe_ids.len() as i64);
            return response_recipes;
        } else {
            start_from = response_recipes.next_start_from;
        }
    }
}

async fn assert_backward_pagination_works(
    limit: i64,
    last_recipe_response: GetRecipesResponse,
    app: App,
    pool: &PgPool,
) {
    let mut queried_recipe_ids = HashSet::<i32>::new();
    let mut start_from: Option<i32> = last_recipe_response.previous_start_from;
    loop {
        let mut query_string = format!("limit={}", limit);
        if let Some(start_id) = start_from {
            query_string = format!("{}&start_from={}", query_string, start_id);
        }
        let query_params = Some(query_string);
        let json = json!({});
        let request = create_get_request_to("recipes", None, query_params, json);
        let response = app.router.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read body bytes");

        let response_recipes: GetRecipesResponse =
            serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

        assert!(response_recipes.recipes.len() as i64 <= limit);

        // Here goes the db assertions
        for recipe in &response_recipes.recipes {
            let rec_id = recipe
                .recipe_id()
                .expect("Recipe id should have been Some(i32)");
            // When doing backwards pagination and overlap is allowed
            // as the query must always fill up the limit.
            let _ = queried_recipe_ids.insert(recipe.recipe_id().unwrap());

            // assert!(newly_inserted);
            assert_detailed_ingredients_exist(&pool, recipe.ingredients())
                .await
                .unwrap();
            assert_recipe_steps_exist(&pool, recipe.steps(), rec_id.clone())
                .await
                .unwrap();
        }
        // End of db assertions
        // After all is done, we can decide whether it is over or not
        if response_recipes.previous_start_from.is_none() {
            let record = sqlx::query!(
                r#"
                    SELECT COUNT(*) as count
                    FROM recipe;
                "#
            )
            .fetch_one(pool)
            .await
            .expect("Should have had a result");
            // You have to remove the starting recipes from the total count
            // because they will not have been re-queried.
            assert_eq!(
                record.count.unwrap() - last_recipe_response.recipes.len() as i64,
                queried_recipe_ids.len() as i64
            );
            return;
        } else {
            start_from = response_recipes.previous_start_from;
        }
    }
}
#[sqlx::test]
async fn getting_recipes_with_wrong_parameters_returns_404_bad_request(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let query_params: Option<String> = Some(format!(
        "{}={}",
        Faker.fake::<String>(),
        Faker.fake::<String>()
    ));
    let json = json!({});
    let request = create_get_request_to("recipes", None, query_params, json);
    let response = app.router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}
