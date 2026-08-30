use anyhow::Error;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::error;
use serde::Serialize;

pub mod connect_error;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error")]
    Database(#[source] anyhow::Error),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            | AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            | AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            | AppError::Database(_) | AppError::Internal(_) => (
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

pub fn log_and_context_error<E>(err: E, message: &str, file: &str, function: &str) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    error!("{function}: {message}: {err}. At {file}");
    Error::new(err).context(message.to_string())
}

#[macro_export]
macro_rules! create_error {
    ($err:expr, $message:expr) => {
        $crate::errors::AppError::Database($crate::errors::log_and_context_error(
            $err,
            $message,
            file!(),
            function_name!(),
        ))
    };
}
