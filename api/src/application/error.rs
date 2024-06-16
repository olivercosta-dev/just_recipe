use core::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, PartialEq)]
pub enum AppError {
    InternalServerError, // DB errors for the most part
    NotFound,            // resource not found
    Conflict,            // resource already exists
    BadRequest,
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
            AppError::BadRequest => StatusCode::BAD_REQUEST,
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

impl fmt::Display for AppError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalServerError => write!(f, "Something unexpected happened. Internal server error."),
            AppError::NotFound => write!(f, "Resource was not found."),
            AppError::Conflict => write!(f, "Conflicting resources."),
            AppError::BadRequest => write!(f, "The request was in incorrect format."),
            AppError::RecipeParsingError(err) => write!(f, "There was an error parsing the recipe: {}", err),
        }
    }
}

impl fmt::Display for RecipeParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecipeParsingError::StepNumbersOutOfOrder => write!(f, "Step numbers are out of order."),
            RecipeParsingError::RecipeIdNotPositive => write!(f, "Recipe ID must be positive."),
            RecipeParsingError::InvalidUnitId => write!(f, "Invalid unit ID."),
            RecipeParsingError::InvalidIngredientId => write!(f, "Invalid ingredient ID."),
            RecipeParsingError::DuplicateIngredientId => write!(f, "Duplicate ingredient ID."),
        }
    }
}
