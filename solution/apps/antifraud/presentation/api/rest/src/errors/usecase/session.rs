use application::usecase::session::error::SessionUseCaseError;

// use axum::http::StatusCode;

// use serde_json::json;
use crate::ApiError;

impl From<SessionUseCaseError> for ApiError {
    fn from(error: SessionUseCaseError) -> Self {
        let (status_code, error_code, error, details) = {
            use SessionUseCaseError as E;
            // use StatusCode as C;
            match error {
                E::Infrastructure(_) => Self::internal_server_error(error),
            }
        };

        Self::UseCase {
            status_code,
            error_code,
            message: error,
            details,
        }
    }
}
