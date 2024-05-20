use std::default;

use axum::http::StatusCode;
use fake::{Fake, Faker};
use just_recipe::{
    application::{app::App, state::AppState},
    unit::Unit,
    utilities::{
        random_generation::units::choose_random_unit, request_creators::create_put_request_to,
    },
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn updating_existing_unit_gets_updated_returns_204_no_content(
    pool: PgPool,
) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let unit_id = choose_random_unit(&app_state.pool).await.unit_id.unwrap();
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();
    let json = json!({"singular_name": singular_name, "plural_name": plural_name});
    let request = create_put_request_to("units", unit_id, json);
    let response = app.router.oneshot(request).await.unwrap();

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

#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn updating_non_existent_unit_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let unit_id = -1; // This is an invalid id by definition.
    let singular_name = Faker.fake::<String>();
    let plural_name = Faker.fake::<String>();
    let json = json!({"singular_name": singular_name, "plural_name": plural_name});
    let request = create_put_request_to("units", unit_id, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}
