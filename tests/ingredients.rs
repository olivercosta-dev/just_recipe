use axum::{
    body::Body,
    http::{request, Request, StatusCode},
};
use fake::{Fake, Faker};
use just_recipe::{
    app::{new_app, AppState},
    routes::Ingredient,
    utils::create_post_request_to,
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test]
async fn adding_new_ingredient_persists_and_returns_200_ok(pool: PgPool) -> sqlx::Result<()> {
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
    assert_eq!(response.status(), StatusCode::OK);
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
