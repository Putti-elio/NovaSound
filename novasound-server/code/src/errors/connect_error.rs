use connectrpc::{ConnectError, error::ErrorDetail};
use novasound_domain::validation::ValidationErrors;

use super::AppError;

/// Error categories exposed through the `ConnectRPC` transport.
pub enum ConnectAppError {
    NotFound(String),
    Validation(ValidationErrors),
    Internal,
}

impl From<AppError> for ConnectAppError {
    fn from(error: AppError) -> Self {
        match error {
            | AppError::NotFound(message) => Self::NotFound(message),
            | AppError::Validation(errors) => Self::Validation(errors),
            | AppError::Database(_) | AppError::Internal(_) => Self::Internal,
        }
    }
}

impl From<ConnectAppError> for ConnectError {
    fn from(error: ConnectAppError) -> Self {
        match error {
            | ConnectAppError::NotFound(message) => Self::not_found(message),
            | ConnectAppError::Validation(errors) => validation_error(errors),
            | ConnectAppError::Internal => Self::internal("Unable to complete the request"),
        }
    }
}

pub fn to_connect_error(error: AppError) -> ConnectError {
    ConnectAppError::from(error).into()
}

/// `ConnectRPC` 0.3.3 exposes generic error details, not a typed validation detail.
/// Validation failures therefore use this stable type URL and a `debug` object with an
/// `errors` list of `{ field, code, message }` entries.
const VALIDATION_ERROR_DETAIL_TYPE_URL: &str = "type.novasound.dev/validation-error-list";

pub fn validation_error(errors: ValidationErrors) -> ConnectError {
    let issues = errors
        .issues()
        .iter()
        .map(|issue| {
            serde_json::json!({
                "field": issue.field,
                "code": issue.code,
                "message": issue.message,
            })
        })
        .collect::<Vec<_>>();

    ConnectError::invalid_argument("Invalid request").with_detail(ErrorDetail {
        type_url: VALIDATION_ERROR_DETAIL_TYPE_URL.to_string(),
        value: None,
        debug: Some(serde_json::json!({ "errors": issues })),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use novasound_domain::validation::ValidationIssue;

    #[test]
    fn maps_not_found_to_connect_not_found() {
        let error = to_connect_error(AppError::NotFound("Artist not found".to_string()));

        assert_eq!(error.code, connectrpc::ErrorCode::NotFound);
    }

    #[test]
    fn maps_validation_to_connect_invalid_argument() {
        let error = to_connect_error(AppError::Validation(
            ValidationIssue::new("name", "already_exists", "Artist already exists").into(),
        ));

        assert_eq!(error.code, connectrpc::ErrorCode::InvalidArgument);
    }

    #[test]
    fn exposes_validation_issues_as_a_stable_detail_list() {
        let error = validation_error(ValidationErrors::from(ValidationIssue::new(
            "name",
            "required",
            "Artist name cannot be empty",
        )));

        assert_eq!(error.code, connectrpc::ErrorCode::InvalidArgument);
        assert_eq!(error.details.len(), 1);
        assert_eq!(error.details[0].type_url, VALIDATION_ERROR_DETAIL_TYPE_URL);
        assert_eq!(
            error.details[0].debug,
            Some(serde_json::json!({
                "errors": [{
                    "field": "name",
                    "code": "required",
                    "message": "Artist name cannot be empty",
                }]
            }))
        );
    }

    #[test]
    fn hides_database_details_from_connect_clients() {
        let error = to_connect_error(AppError::Database(anyhow::anyhow!("connection refused")));

        assert_eq!(error.code, connectrpc::ErrorCode::Internal);
        assert_eq!(
            error.message.as_deref(),
            Some("Unable to complete the request")
        );
    }
}
