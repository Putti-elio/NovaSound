use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::sync::MutexGuard;
use std::sync::PoisonError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Mutex poisoned")]
    MutexPoisoned,
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for AppError {
    fn from(_: PoisonError<MutexGuard<'_, T>>) -> Self {
        AppError::MutexPoisoned
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string(),
            ),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::MutexPoisoned => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error".to_string(),
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
