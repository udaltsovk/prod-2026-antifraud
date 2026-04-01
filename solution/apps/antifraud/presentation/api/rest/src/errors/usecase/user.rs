use application::usecase::user::UserUseCaseError;
use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::ApiError;

impl From<UserUseCaseError> for ApiError {
    fn from(error: UserUseCaseError) -> Self {
        let (status_code, error_code, error, details) = {
            use StatusCode as C;
            use UserUseCaseError as E;
            match error {
                E::Infrastructure(_) => Self::internal_server_error(error),

                E::Validation(err) => return Self::from(err),

                E::EmailAlreadyUsed(ref email) => (
                    C::CONFLICT,
                    "EMAIL_ALREADY_EXISTS",
                    error.to_string(),
                    json!({
                        "field": "email",
                        "value": email.to_string()
                    }),
                ),

                E::InvalidPassword => (
                    StatusCode::UNAUTHORIZED,
                    "UNAUTHORIZED",
                    "Invalid credentials".to_string(),
                    Value::Null,
                ),

                E::UserDeactivated => {
                    (C::LOCKED, "USER_INACTIVE", error.to_string(), Value::Null)
                },

                E::NotFoundByEmail(_) | E::NotFoundById(_) => {
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
