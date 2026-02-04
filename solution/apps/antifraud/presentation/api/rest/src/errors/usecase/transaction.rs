use application::usecase::transaction::error::TransactionUseCaseError;
use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::ApiError;

impl From<TransactionUseCaseError> for ApiError {
    fn from(error: TransactionUseCaseError) -> Self {
        let (status_code, error_code, error, details) = {
            use StatusCode as C;
            use TransactionUseCaseError as E;
            match error {
                E::Infrastructure(_) => Self::internal_server_error(error),

                E::Validation(err) => return Self::from(err),

                E::UserDeactivated => {
                    (C::LOCKED, "USER_INACTIVE", error.to_string(), Value::Null)
                },

                E::UserNotFoundById(id) => (
                    C::NOT_FOUND,
                    "NOT_FOUND",
                    error.to_string(),
                    json!({
                        "userId": id
                    }),
                ),
                E::TransactionNotFoundById(..) => {
                    (C::NOT_FOUND, "NOT_FOUND", error.to_string(), Value::Null)
                },

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
