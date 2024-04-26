use axum::{body::Body, http::{Request, StatusCode}};
use fake::{Fake, Faker};
use just_recipe::{app::{new_app, AppState}, routes::Ingredient};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
#[sqlx::test]
async fn adding_new_ingredient_persists_and_returns_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/ingredients")
                .header("Content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(
                        &json!({"singular_name":singular_name, "plural_name":plural_name}),
                    )
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

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
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/ingredients")
                .header("Content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(
                        &json!({"singular_name":singular_name, "plural_name":plural_name}),
                    )
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
    Ok(())
}
