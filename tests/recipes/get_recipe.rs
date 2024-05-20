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

#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn getting_recipes_returns_recipes_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let limit: i64 = (1..=15).fake(); // max 15 recipes at a time, otherwise it would be an error
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
        let start_id = if let Some(start_id) = start_from {
            start_id
        } else {
            -1
        };

        // Here goes the db assertions
        for recipe in response_recipes.recipes {
            let rec_id = recipe
                .recipe_id()
                .expect("Recipe id should have been Some(i32)");
            let already_exists = queried_recipe_ids.insert(recipe.recipe_id().unwrap());
            assert!(!already_exists); // The endpoint should never give overlapping results (if the queries are consisten)
            assert_detailed_ingredients_exist(&app_state.pool, recipe.ingredients())
                .await
                .unwrap();
            assert_recipe_steps_exist(&app_state.pool, recipe.steps(), rec_id.clone())
                .await
                .unwrap();
        }
        // End of db assertions
        // After all is done, we can decide whether it is over or not
        if response_recipes.next_start_from.is_none() {
            break;
        } else {
            start_from = response_recipes.next_start_from;
        }
    }

    Ok(())
}


#[sqlx::test]
async fn getting_recipes_with_wrong_parameters_returns_404_bad_request(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let query_params: Option<String> = Some(format!("{}={}", Faker.fake::<String>(), Faker.fake::<String>()));
    let json = json!({});
    let request = create_get_request_to("recipes", None, query_params, json);
    let response = app.router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}