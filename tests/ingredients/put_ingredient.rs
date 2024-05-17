use std::default;

use axum::http::StatusCode;
use fake::{Fake, Faker};
use just_recipe::{application::{app::App, state::AppState}, ingredient::Ingredient, unit::Unit};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use crate::{choose_random_ingredient, create_put_request_to};
#[sqlx::test(fixtures(path = "../fixtures", scripts("ingredients")))]
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

#[sqlx::test(fixtures(path = "../fixtures", scripts("ingredients")))]
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
