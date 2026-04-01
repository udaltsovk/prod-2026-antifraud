use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        impl_try_from_external_input,
        validation::{
            Constraints,
            error::{ValidationErrors, ValidationResult},
        },
    },
    redact::Secret,
};

use crate::{constraints::PASSWORD_LENGTH_CONSTRAINTS, password::Password};

#[derive(DomainType, Clone, Debug)]
pub struct SessionPassword(Secret<String>);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with(&PASSWORD_LENGTH_CONSTRAINTS).build()
});

impl TryFrom<Secret<String>> for SessionPassword {
    type Error = ValidationErrors;

    fn try_from(value: Secret<String>) -> ValidationResult<Self> {
        CONSTRAINTS
            .check(value.expose_secret())
            .into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = SessionPassword,
    input_type = Secret<String>
);

impl From<SessionPassword> for Password {
    fn from(password: SessionPassword) -> Self {
        Self(password.0)
    }
}
