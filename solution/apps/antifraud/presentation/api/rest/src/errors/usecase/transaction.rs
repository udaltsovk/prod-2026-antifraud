use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::transaction::error::TransactionUseCaseError,
};
use axum::http::StatusCode;
use serde_json::Value;

use crate::ApiError;

impl<R, S> From<TransactionUseCaseError<R, S>> for ApiError
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn from(error: TransactionUseCaseError<R, S>) -> Self {
        let (status_code, error_code, error, details) = {
            use StatusCode as C;
            use TransactionUseCaseError as E;
            match error {
                E::Repository(_) | E::Service(_) => {
                    Self::internal_server_error(error)
                },

                E::Validation(err) => return Self::from(err),

                E::UserDeactivated => {
                    (C::LOCKED, "USER_INACTIVE", error.to_string(), Value::Null)
                },

                E::UserNotFoundById(..) | E::TransactionNotFoundById(..) => {
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
