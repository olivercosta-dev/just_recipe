use std::default;

use axum::{body::to_bytes, http::StatusCode};
use fake::Fake;
use just_recipe::{application::{app::App, state::AppState}, routes::GetUnitsResponse, unit::Unit};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use crate::{choose_random_unit, create_get_request_to};
#[sqlx::test(fixtures(path = "../fixtures", scripts("units")))]
async fn getting_existing_unit_returns_ingredient_and_200_ok(pool: PgPool) -> sqlx::Result<()> {
    let app_state = AppState::new(pool);
    let app = App::new(app_state.clone(), default::Default::default(), 0).await;
    let unit = choose_random_unit(&app_state.pool).await;
    let json = json!({}); // this is not needed for a get
    let request = create_get_request_to("units", Some(unit.unit_id.unwrap()),None, json);
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
    let request = create_get_request_to("units", Some(-1),None, json);
    let response = app.router.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

// TODO (oliver) Make all the sad paths!
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
        assert_eq!(
            response_units.units.len(),
            units_in_db.len()
        );

        assert_units_match(&response_units.units, &units_in_db);
        if response_units.next_start_from.is_none() {
            break;
        } else {
            start_from = response_units.next_start_from;
        }
    }

    Ok(())
}

fn assert_units_match(left_units: &[Unit], right_units: &[Unit]) {
    // Ensure both unit slices are sorted by unit_id
    let mut left_sorted = left_units.to_vec();
    let mut right_sorted = right_units.to_vec();

    left_sorted.sort_by_key(|unit| unit.unit_id);
    right_sorted.sort_by_key(|unit| unit.unit_id);

    assert_eq!(
        left_sorted.len(),
        right_sorted.len(),
        "The number of ingredients does not match."
    );

    for (left, right) in left_sorted.iter().zip(right_sorted.iter()) {
        assert_eq!(
            left.unit_id, right.unit_id,
            "Unit ID mismatch: left = {:?}, right = {:?}",
            left, right
        );
        assert_eq!(
            left.singular_name, right.singular_name,
            "Singular name mismatch: left = {:?}, right = {:?}",
            left, right
        );
        assert_eq!(
            left.plural_name, right.plural_name,
            "Plural name mismatch: left = {:?}, right = {:?}",
            left, right
        );
    }
}
