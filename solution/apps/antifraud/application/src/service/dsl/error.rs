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
    pub position: Option<u64>,
    pub near: Option<String>,
}

pub type DslServiceResult<T> = Result<T, Vec<DslServiceError>>;
