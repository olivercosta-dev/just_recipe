use axum::{extract::State, http::StatusCode,Json};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, query};

use crate::app::AppState;

#[derive(Deserialize, Serialize, Debug)]
pub struct Unit {
    #[serde(skip)]
    pub unit_id: i32,
    pub singular_name: String,
    pub plural_name: String,
}

pub async fn units(State(app_state) : State<AppState>, Json(unit): Json<Unit>) -> StatusCode {
    let result: Result<PgQueryResult, sqlx::Error> = query!(
        r#"
            INSERT INTO unit (singular_name, plural_name) 
            VALUES ($1, $2);
        "#,
        unit.singular_name,
        unit.plural_name
    )
    .execute(&app_state.pool)
    .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(sqlx::Error::Database(_)) => {
            StatusCode::CONFLICT
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    } 
}