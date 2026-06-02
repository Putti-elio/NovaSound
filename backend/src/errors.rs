use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            | AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            | AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            | AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: message,
            status: status.as_u16(),
        });

        (status, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    status: u16,
}

pub type AppResult<T> = Result<T, AppError>;
