use std::sync::LazyLock;

use derive_more::From;
use lib::{
    DomainType,
    domain::validation::{
        Constraints, ExternalInput,
        error::{ValidationErrors, ValidationResult},
    },
    tap::Pipe as _,
};

use crate::constraints::PASSWORD_CONSTRAINTS;

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Password(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("password", &PASSWORD_CONSTRAINTS).build()
});

impl TryFrom<String> for Password {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl TryFrom<ExternalInput<String>> for Password {
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
impl Password {
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(From)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PasswordHash(pub String);
