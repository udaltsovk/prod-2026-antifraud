use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::user::error::UserUseCaseError,
};
use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::ApiError;

impl<R, S> From<UserUseCaseError<R, S>> for ApiError
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn from(error: UserUseCaseError<R, S>) -> Self {
        let (status_code, error_code, error, details) = {
            use StatusCode as C;
            use UserUseCaseError as E;
            match error {
                E::Repository(_) | E::Service(_) => {
                    Self::internal_server_error(error)
                },

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

                E::InvalidPassword => Self::invalid_credentials(error),

                E::NotFoundByEmail {
                    from_auth, ..
                } if from_auth => Self::invalid_credentials(error),

                E::UserDeactivated => {
                    (C::LOCKED, "USER_INACTIVE", error.to_string(), Value::Null)
                },

                E::NotFoundByEmail {
                    ..
                }
                | E::NotFoundById(..) => {
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
