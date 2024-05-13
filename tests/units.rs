use axum::body::to_bytes;
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
    let app_state = AppState::new(pool).await;
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
    let app_state = AppState::new(pool).await;
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
    let app_state = AppState::new(pool).await;
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
async fn deleting_non_existent_unit_returns_404_not_found(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool).await;
    let app = new_app(app_state.clone()).await;
    let unit_id = -1;
    let request = create_delete_request_to("units", json!({"unit_id": unit_id}));
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}


#[sqlx::test(fixtures("units"))]
async fn updating_existing_unit_gets_updated_returns_204_no_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool).await;
    let app = new_app(app_state.clone()).await;
    let unit_id = choose_random_unit(&app_state.pool).await.unit_id.unwrap();
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
async fn updating_non_existent_unit_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool).await;
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


#[sqlx::test(fixtures("units"))]
async fn getting_existing_unit_returns_ingredient_and_200_ok(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool).await;
    let app = new_app(app_state.clone()).await;
    let unit = choose_random_unit(&app_state.pool).await;
    let json = json!({}); // this is not needed for a get
    let request = create_get_request_to("units", unit.unit_id.unwrap(), json);
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body bytes");

    let response_unit: Unit =
        serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

    assert_eq!(response_unit.unit_id, unit.unit_id);
    assert_eq!(response_unit.singular_name, unit.singular_name);
    assert_eq!(response_unit.plural_name, unit.plural_name);
    Ok(())
}
#[sqlx::test(fixtures("units"))]
async fn getting_non_existent_unit_returns_404_not_found(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool).await;
    let app = new_app(app_state.clone()).await;
    let json = json!({}); // won't be needing this
    let request = create_get_request_to("units", -1, json);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
