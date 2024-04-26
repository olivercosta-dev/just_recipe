use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use fake::{Fake, Faker};
use just_recipe::{app::{new_app, AppState}, routes::Unit};

use serde_json::json;
use sqlx::{PgPool};
use tower::ServiceExt; // for `oneshot`


#[sqlx::test]
async fn adding_new_unit_persists_and_returns_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/units")
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
        Unit,
        r#"
            SELECT unit_id, singular_name, plural_name
            FROM unit
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

// async fn adding_existing_unit_returns_error() {
//     let app = new_app("postgres://postgres@localhost/just_recipe").await;
//     let singular_name = "kilogram";
//     let plural_name = "kilograms";

//     let response = app
//         .oneshot(
//             Request::builder()
//                 .method("POST")
//                 .uri("/units")
//                 .header("Content-type", "application/json")
//                 .body(Body::from(
//                     serde_json::to_vec(
//                         &json!({"singular_name":singular_name, "plural_name":plural_name}),
//                     )
//                     .unwrap(),
//                 ))
//                 .unwrap(),
//         )
//         .await
//         .unwrap();

//     let pool = PgPoolOptions::new()
//         .connect("postgres://postgres@localhost/just_recipe")
//         .await
//         .unwrap();
//     let query_result = sqlx::query_as!(
//         Unit,
//         r#"
//             SELECT unit_id, singular_name, plural_name
//             FROM unit
//             WHERE singular_name = $1 AND plural_name = $2;
//         "#,
//         singular_name,
//         plural_name
//     )
//         .fetch_one(&pool)
//         .await
//         .unwrap();
// }