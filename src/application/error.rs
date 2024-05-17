use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, PartialEq)]
pub enum AppError {
    InternalServerError,
    NotFound,
    Conflict,
    RecipeParsingError(RecipeParsingError),
}

#[derive(Debug, PartialEq)]
pub enum RecipeParsingError {
    StepNumbersOutOfOrder,
    RecipeIdNotPositive,
    InvalidUnitId,
    InvalidIngredientId,
    DuplicateIngredientId,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::RecipeParsingError(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
        .into_response()
    }
}
impl From<RecipeParsingError> for AppError {
    fn from(err: RecipeParsingError) -> Self {
        AppError::RecipeParsingError(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        AppError::InternalServerError
    }
}
