use application::usecase::fraud_rule::error::FraudRuleUseCaseError;
use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::ApiError;

impl From<FraudRuleUseCaseError> for ApiError {
    fn from(error: FraudRuleUseCaseError) -> Self {
        let (status_code, error_code, error, details) = {
            use FraudRuleUseCaseError as E;
            use StatusCode as C;
            match error {
                E::Infrastructure(_) => Self::internal_server_error(error),

                E::Validation(err) => return Self::from(err),

                E::NameAlreadyUsed(ref name) => (
                    C::CONFLICT,
                    "RULE_NAME_ALREADY_EXISTS",
                    error.to_string(),
                    json!({
                        "field": "name",
                        "value": name.to_string()
                    }),
                ),

                E::NotFoundByName(..) | E::NotFoundById(..) => {
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
