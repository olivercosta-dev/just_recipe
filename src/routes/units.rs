use axum::{extract::{Path, State}, http::StatusCode,Json};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveUnitRequest {
    pub unit_id: i32,
}
pub async fn add_unit(State(app_state) : State<AppState>, Json(unit): Json<Unit>) -> StatusCode {
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
        Ok(_) => StatusCode::NO_CONTENT,
        Err(sqlx::Error::Database(_)) => {
            StatusCode::CONFLICT
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    } 
}

pub async fn remove_unit(
    State(app_state): State<AppState>,
    Json(delete_unit_request): Json<RemoveUnitRequest>,
) -> StatusCode {
    match sqlx::query!(
        "DELETE FROM unit WHERE unit_id = $1",
        delete_unit_request.unit_id
    )
    .execute(&app_state.pool)
    .await{
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn update_unit(
    State(app_state): State<AppState>,
    Path(unit_id): Path<i32>,
    Json(unit): Json<Unit>,
) -> StatusCode {
    match sqlx::query!(
        "UPDATE unit SET singular_name = $1, plural_name = $2 WHERE unit_id = $3",
        unit.singular_name,
        unit.plural_name,
        unit_id,
    )
    .execute(&app_state.pool)
    .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::NO_CONTENT
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}