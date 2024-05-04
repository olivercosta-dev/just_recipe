use axum::http::StatusCode;
use fake::{Fake, Faker};
use just_recipe::{
    app::{new_app, AppState},
    routes::Ingredient,
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
mod utils;
use utils::*;

#[sqlx::test]
async fn adding_new_ingredient_persists_returns_204_no_content(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();
    let json = json!({"singular_name":singular_name, "plural_name":plural_name});
    let request = create_post_request_to("ingredients", json);
    let response = app.oneshot(request).await.unwrap();

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
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let singular_name = "carrot";
    let plural_name = "carrots";
    let json = json!({"singular_name":singular_name, "plural_name":plural_name});
    let request = create_post_request_to("ingredients", json);
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
    Ok(())
}

#[sqlx::test(fixtures("ingredients"))]
async fn deleting_existing_ingredient_gets_removed_returns_204_no_content(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let ingredient_id = choose_random_ingredient(&app_state.pool).await.ingredient_id;
    let request = create_delete_request_to("ingredients", json!({"ingredient_id": ingredient_id}));
    let response = app.oneshot(request).await.unwrap();
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