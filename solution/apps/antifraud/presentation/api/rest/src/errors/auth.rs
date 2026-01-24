use axum::http::StatusCode;
use lib::presentation::api::rest::errors::InternalErrorStringExt as _;
use serde_json::Value;

use crate::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Токен отсутствует или невалиден")]
    InvalidToken,

    #[error("Недостаточно прав для выполнения операции")]
    MissingPermissions,
}

impl ApiError {
    pub fn invalid_credentials<T>(
        error: T,
    ) -> (StatusCode, &'static str, String, Value)
    where
        T: ToString,
    {
        (
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED",
            error.to_internal_error_string("Invalid credentials"),
            Value::Null,
        )
    }
}

impl From<AuthError> for ApiError {
    fn from(error: AuthError) -> Self {
        let (status_code, error_code, error) = {
            use AuthError as E;
            use StatusCode as C;
            match error {
                E::InvalidToken => {
                    (C::UNAUTHORIZED, "UNAUTHORIZED", error.to_string())
                },
                E::MissingPermissions => {
                    (C::FORBIDDEN, "FORBIDDEN", error.to_string())
                },
            }
        };

        Self::UseCase {
            status_code,
            error_code,
            message: error,
            details: Value::Null,
        }
    }
}
