use connectrpc::ConnectError;

use super::AppError;

/// Error categories exposed through the `ConnectRPC` transport.
pub enum ConnectAppError {
    NotFound(String),
    Validation(String),
    Internal,
}

impl From<AppError> for ConnectAppError {
    fn from(error: AppError) -> Self {
        match error {
            | AppError::NotFound(message) => Self::NotFound(message),
            | AppError::Validation(message) => Self::Validation(message),
            | AppError::Database(_) | AppError::Internal(_) => Self::Internal,
        }
    }
}

impl From<ConnectAppError> for ConnectError {
    fn from(error: ConnectAppError) -> Self {
        match error {
            | ConnectAppError::NotFound(message) => Self::not_found(message),
            | ConnectAppError::Validation(message) => Self::invalid_argument(message),
            | ConnectAppError::Internal => Self::internal("Unable to complete the request"),
        }
    }
}

impl From<AppError> for ConnectError {
    fn from(error: AppError) -> Self {
        ConnectAppError::from(error).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_not_found_to_connect_not_found() {
        let error = ConnectError::from(AppError::NotFound("Artist not found".to_string()));

        assert_eq!(error.code, connectrpc::ErrorCode::NotFound);
    }

    #[test]
    fn maps_validation_to_connect_invalid_argument() {
        let error = ConnectError::from(AppError::Validation("Artist already exists".to_string()));

        assert_eq!(error.code, connectrpc::ErrorCode::InvalidArgument);
    }

    #[test]
    fn hides_database_details_from_connect_clients() {
        let error = ConnectError::from(AppError::Database(anyhow::anyhow!("connection refused")));

        assert_eq!(error.code, connectrpc::ErrorCode::Internal);
        assert_eq!(
            error.message.as_deref(),
            Some("Unable to complete the request")
        );
    }
}
