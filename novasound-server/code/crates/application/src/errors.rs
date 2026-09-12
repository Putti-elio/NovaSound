use anyhow::Error;
use log::error;
use novasound_domain::validation::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(ValidationErrors),
    #[error("Database error")]
    Database(#[source] anyhow::Error),
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
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
