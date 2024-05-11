use axum::http::StatusCode;
use fake::{Fake, Faker};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

mod utils;
use utils::*;
use just_recipe::app::{new_app, AppState};
use just_recipe::unit::Unit;

#[sqlx::test]
async fn adding_new_unit_persists_and_returns_204_no_content(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();
    let json = json!({"singular_name":singular_name, "plural_name":plural_name});
    let request = create_post_request_to("units", json);
    let response = app.oneshot(request).await.unwrap();

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
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}

#[sqlx::test(fixtures("units"))]
async fn adding_existing_unit_returns_409_conflict(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let singular_name = "kilogram";
    let plural_name = "kilograms";
    let json = json!({"singular_name":singular_name, "plural_name":plural_name});
    let request = create_post_request_to("units", json);

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
    Ok(())
}

#[sqlx::test(fixtures("units"))]
async fn deleting_existing_unit_gets_removed_returns_204_no_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let unit_id = choose_random_unit(&app_state.pool).await.unit_id;
    let request = create_delete_request_to("units", json!({"unit_id": unit_id}));
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let unit_record = sqlx::query!("SELECT unit_id from unit where unit_id = $1", unit_id)
        .fetch_optional(&app_state.pool)
        .await
        .unwrap();
    assert!(unit_record.is_none());
    Ok(())
}
#[sqlx::test(fixtures("units"))]
async fn updating_existing_unit_gets_updated_returns_204_no_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let unit_id = choose_random_unit(&app_state.pool).await.unit_id;
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();
    let json = json!({"singular_name": singular_name, "plural_name": plural_name});
    let request = create_put_request_to("units", unit_id, json);
    let response = app.oneshot(request).await.unwrap();

    let query_result = sqlx::query_as!(
        Unit,
        r#"
            SELECT unit_id, singular_name, plural_name
            FROM unit
            WHERE unit_id = $1
        "#,
        unit_id,
    )
    .fetch_one(&app_state.pool)
    .await
    .expect("Unit id should have existed");
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(
        (query_result.singular_name, query_result.plural_name),
        (singular_name, plural_name)
    );
    Ok(())
}

#[sqlx::test(fixtures("units"))]
async fn updating_non_existing_unit_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState { pool };
    let app = new_app(app_state.clone()).await;
    let unit_id = -1; // This is an invalid id by definition.
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();
    let json = json!({"singular_name": singular_name, "plural_name": plural_name});
    let request = create_put_request_to("units", unit_id, json);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
