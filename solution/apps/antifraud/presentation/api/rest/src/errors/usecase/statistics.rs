use application::usecase::statistics::error::StatisticsUseCaseError;
use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::ApiError;

impl From<StatisticsUseCaseError> for ApiError {
    fn from(error: StatisticsUseCaseError) -> Self {
        let (status_code, error_code, error, details) = {
            use StatisticsUseCaseError as E;
            use StatusCode as C;
            match error {
                E::Infrastructure(_) => Self::internal_server_error(error),

                E::Validation(err) => return Self::from(err),

                E::UserNotFoundById(id) => (
                    C::NOT_FOUND,
                    "NOT_FOUND",
                    error.to_string(),
                    json!({
                        "userId": id
                    }),
                ),

                E::MissingPermissions => {
                    (C::FORBIDDEN, "FORBIDDEN", error.to_string(), Value::Null)
                },
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
