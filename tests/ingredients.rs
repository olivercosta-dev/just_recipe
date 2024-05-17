use std::default;

use axum::{body::to_bytes, http::StatusCode};
use fake::{Fake, Faker};
use just_recipe::{application::{app::App, state::AppState}, ingredient::Ingredient};
use serde_json::json;
use sqlx::PgPool;
mod utils;
use tower::ServiceExt;
use utils::*;

#[sqlx::test]
async fn adding_new_ingredient_persists_returns_204_no_content(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();
    let json = json!({"singular_name":singular_name, "plural_name":plural_name});
    let request = create_post_request_to("ingredients", json);
    let response = app.router.oneshot(request).await.unwrap();

    let query_result = sqlx::query_as!(
        Ingredient,
        r#"
            SELECT ingredient_id, singular_name, plural_name
            FROM ingredient
            WHERE singular_name = $1 AND plural_name = $2;
        "#,
        singular_name,
        plural_name
    )
    .fetch_one(&app_state.pool)
    .await
    .unwrap();
    assert_eq!(query_result.singular_name, singular_name);
    assert_eq!(query_result.plural_name, plural_name);
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}
#[sqlx::test(fixtures("ingredients"))]
async fn adding_existing_ingredient_returns_409_conflict(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let singular_name = "carrot";
    let plural_name = "carrots";
    let json = json!({"singular_name":singular_name, "plural_name":plural_name});
    let request = create_post_request_to("ingredients", json);
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
    Ok(())
}

#[sqlx::test(fixtures("ingredients"))]
async fn deleting_non_existent_ingredient_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let ingredient_id = -1;
    let request = create_delete_request_to("ingredients", json!({"ingredient_id": ingredient_id}));
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
#[sqlx::test(fixtures("ingredients"))]
async fn deleting_existing_ingredient_gets_removed_returns_204_no_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let ingredient_id = choose_random_ingredient(&app_state.pool)
        .await
        .ingredient_id;
    let request = create_delete_request_to("ingredients", json!({"ingredient_id": ingredient_id}));
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let ingredient_record = sqlx::query!(
        "SELECT ingredient_id from ingredient where ingredient_id = $1",
        ingredient_id
    )
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();
    assert!(ingredient_record.is_none());
    Ok(())
}

#[sqlx::test(fixtures("ingredients"))]
async fn updating_existing_ingredient_gets_updated_returns_204_no_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    
    let ingredient_id = choose_random_ingredient(&app_state.pool)
        .await
        .ingredient_id
        .expect("should have been able to unwrap ingredent_id");

    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();
    let json = json!({"singular_name": singular_name, "plural_name": plural_name});
    let request = create_put_request_to("ingredients", ingredient_id, json);
    let response = app.router.oneshot(request).await.unwrap();

    let query_result = sqlx::query_as!(
        Ingredient,
        r#"
            SELECT ingredient_id, singular_name, plural_name
            FROM ingredient
            WHERE ingredient_id = $1
        "#,
        ingredient_id,
    )
    .fetch_one(&app_state.pool)
    .await
    .expect("Ingredient id should have existed");
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(
        (query_result.singular_name, query_result.plural_name),
        (singular_name, plural_name)
    );
    Ok(())
}

#[sqlx::test(fixtures("ingredients"))]
async fn updating_non_existent_ingredient_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let ingredient_id = -1; // This ingredient_id will never exist, as they are always positive
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();
    let json = json!({"singular_name": singular_name, "plural_name": plural_name});
    let request = create_put_request_to("ingredients", ingredient_id, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[sqlx::test(fixtures("ingredients"))]
async fn getting_existing_ingredient_returns_ingredient_and_200_ok(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let ingredient = choose_random_ingredient(&app_state.pool).await;
    let json = json!({}); // this is not needed for a get
    let request = create_get_request_to("ingredients", ingredient.ingredient_id.unwrap(), json);
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
#[sqlx::test(fixtures("ingredients"))]
async fn getting_non_existent_ingredient_returns_404_not_found(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let json = json!({}); // won't be needing this
    let request = create_get_request_to("ingredients", -1, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
