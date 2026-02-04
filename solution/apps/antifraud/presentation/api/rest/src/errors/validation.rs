use lib::{
    domain::validation::error::ValidationErrorsWithFields,
    presentation::api::rest::{
        errors::validation::FieldErrors, validation_error_response,
    },
};

use crate::errors::ApiError;

validation_error_response!(
    /// поля не прошли валидацию
    name = ValidationFailedResponse,
    error_code = "VALIDATION_FAILED",
    status_code = UNPROCESSABLE_ENTITY,
);

impl From<FieldErrors> for ValidationFailedResponse {
    fn from(errors: FieldErrors) -> Self {
        Self::new("Some fields haven't passed validation", errors)
    }
}

impl From<ValidationErrorsWithFields> for ApiError {
    fn from(errors: ValidationErrorsWithFields) -> Self {
        Self::Validation(errors.into())
    }
}
