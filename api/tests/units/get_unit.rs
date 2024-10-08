use std::default;

use axum::{body::to_bytes, http::StatusCode};
use fake::{Fake, Faker};
use just_recipe::{
    application::{app::App, state::AppState},
    routes::GetUnitsResponse,
    unit::Unit,
    utilities::{
        assertions::assert_units_match, random_generation::units::choose_random_unit, request_creators::create_get_request_to
    },
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn getting_existing_unit_returns_ingredient_and_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let unit = choose_random_unit(&app_state.pool).await;
    let json = json!({}); // this is not needed for a get
    let request = create_get_request_to("units", Some(unit.unit_id.unwrap()), None, json);
    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body bytes");

    let response_unit: Unit = serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

    assert_eq!(response_unit.unit_id, unit.unit_id);
    assert_eq!(response_unit.singular_name, unit.singular_name);
    assert_eq!(response_unit.plural_name, unit.plural_name);
    Ok(())
}
#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn getting_non_existent_unit_returns_404_not_found(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let json = json!({}); // won't be needing this
    let request = create_get_request_to("units", Some(-1), None, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn getting_units_returns_units_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let limit: i64 = (1..=15).fake();
    let mut start_from: Option<i32> = None;
    loop {
        let mut query_string = format!("limit={}", limit);
        if let Some(start_id) = start_from {
            query_string = format!("{}&start_from={}", query_string, start_id);
        }
        let query_params = Some(query_string);
        let json = json!({});
        let request = create_get_request_to("units", None, query_params, json);
        let response = app.router.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read body bytes");

        let response_units: GetUnitsResponse =
            serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

        assert!(response_units.units.len() as i64 <= limit);
        let start_id = if let Some(start_id) = start_from {
            start_id
        } else {
            -1
        };
        let units_in_db = sqlx::query_as!(
            Unit,
            r#" 
                SELECT * 
                FROM unit
                WHERE unit_id >= $1
                ORDER BY unit_id
                LIMIT $2
            "#,
            start_id,
            limit
        )
        .fetch_all(&app_state.pool)
        .await
        .unwrap();
        assert_eq!(response_units.units.len(), units_in_db.len());

        assert_units_match(&response_units.units, &units_in_db);
        if response_units.next_start_from.is_none() {
            break;
        } else {
            start_from = response_units.next_start_from;
        }
    }

    Ok(())
}

#[sqlx::test]
async fn getting_units_with_wrong_parameters_returns_404_bad_request(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let query_params: Option<String> = Some(format!("{}={}", Faker.fake::<String>(), Faker.fake::<String>()));
    let json = json!({});
    let request = create_get_request_to("units", None, query_params, json);
    let response = app.router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    Ok(())
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn getting_all_units_returns_all_units(pool: PgPool) -> sqlx::Result<()> {
    let request = create_get_request_to("units/all", None, None, json!({}));
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let response = app
        .router
        .oneshot(request)
        .await
        .expect("should have gotten a response");
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read body bytes");
    let response_units: Vec<Unit> =
        serde_json::from_slice(&bytes).expect("Failed to deserialize JSON");

    let db_units = sqlx::query_as!(
        Unit,
        r#"
            SELECT *
            FROM unit
            ORDER BY singular_name
        "#,
    )
    .fetch_all(&app_state.pool)
    .await?;
    assert_eq!(response_units.len(), db_units.len());
    // Because both should be in order by singular_name
    db_units
        .iter()
        .zip(response_units.iter())
        .for_each(|(db, resp)| {
            assert_eq!(db, resp);
        });
    Ok(())
}