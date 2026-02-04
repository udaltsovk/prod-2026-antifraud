use application::service::dsl::{DslServiceError, DslServiceErrorKind};
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct DslErrorDto {
    pub code: &'static str,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub near: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
}

impl From<DslServiceError> for DslErrorDto {
    fn from(error: DslServiceError) -> Self {
        let DslServiceError {
            kind,
            message,
            position,
            near,
        } = error;

        let code = {
            use DslServiceErrorKind as K;
            match kind {
                K::ParseError => "DSL_PARSE_ERROR",
                K::InvalidField => "DSL_INVALID_FIELD",
                K::InvalidOperator => "DSL_INVALID_OPERATOR",
            }
        };

        Self {
            code,
            message,
            near,
            position,
        }
    }
}
