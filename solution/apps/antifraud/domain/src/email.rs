use std::{fmt, sync::LazyLock};

use lib::{
    DomainType,
    domain::validation::{
        Constraints, ExternalInput,
        error::{ValidationErrors, ValidationResult},
    },
    tap::Pipe as _,
};

use crate::constraints::EMAIL_CONSTRAINTS;

#[derive(DomainType, Debug)]
pub struct Email(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("email", &EMAIL_CONSTRAINTS).build()
});

impl TryFrom<String> for Email {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS
            .check(&value)
            .into_result(|_| value.to_lowercase().pipe(Self))
    }
}

impl TryFrom<ExternalInput<String>> for Email {
    type Error = ValidationErrors;

    fn try_from(input: ExternalInput<String>) -> ValidationResult<Self> {
        input.map_or_else(
            Self::try_from,
            |input| CONSTRAINTS.type_mismatch_error(input).pipe(Err),
            || CONSTRAINTS.none_error().pipe(Err),
            || CONSTRAINTS.missing_error().pipe(Err),
        )
    }
}
impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
