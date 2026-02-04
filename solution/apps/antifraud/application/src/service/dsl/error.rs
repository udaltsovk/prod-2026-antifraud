use dsl::{ParserError, ValidatorError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DslServiceErrorKind {
    ParseError,
    InvalidField,
    InvalidOperator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DslServiceError {
    pub kind: DslServiceErrorKind,
    pub message: String,
    pub position: Option<usize>,
    pub near: Option<String>,
}

pub type DslServiceErrors = Vec<DslServiceError>;
pub type DslServiceResult<T> = Result<T, DslServiceErrors>;

impl From<ParserError<'_>> for DslServiceError {
    fn from(
        ParserError {
            message,
            near,
            position,
        }: ParserError<'_>,
    ) -> Self {
        Self {
            kind: DslServiceErrorKind::ParseError,
            message,
            position: Some(position),
            near: Some(near.to_string()),
        }
    }
}

impl From<ValidatorError<'_>> for DslServiceError {
    fn from(error: ValidatorError<'_>) -> Self {
        let (kind, message) = match error {
            ValidatorError::InvalidField {
                field,
            } => (
                DslServiceErrorKind::InvalidField,
                format!("Invalid field: `{field}`"),
            ),
            ValidatorError::InvalidOperator {
                field,
                op,
                value,
            } => (
                DslServiceErrorKind::InvalidOperator,
                format!(
                    "Operator `{op}` cannot be used between field `{field}` and value `{value}`"
                ),
            ),
        };

        Self {
            kind,
            message,
            position: None,
            near: None,
        }
    }
}

trait DslServiceErrorExtSeal {}

#[expect(private_bounds, reason = "we're using the seal pattern here")]
pub trait DslServiceErrorExt: DslServiceErrorExtSeal {
    fn into_dsl_service_errors(self) -> DslServiceErrors;
}

impl DslServiceErrorExtSeal for ParserError<'_> {}
impl DslServiceErrorExt for ParserError<'_> {
    fn into_dsl_service_errors(self) -> DslServiceErrors {
        vec![self.into()]
    }
}

impl DslServiceErrorExtSeal for Vec<ValidatorError<'_>> {}
impl DslServiceErrorExt for Vec<ValidatorError<'_>> {
    fn into_dsl_service_errors(self) -> DslServiceErrors {
        self.into_iter().map(From::from).collect()
    }
}
