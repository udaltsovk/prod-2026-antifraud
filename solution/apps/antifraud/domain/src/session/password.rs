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
};

use crate::constraints::PASSWORD_LENGTH_CONSTRAINTS;

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SessionPassword(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("password", &PASSWORD_LENGTH_CONSTRAINTS).build()
});

impl TryFrom<String> for SessionPassword {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = SessionPassword,
    input_type = String,
    constraints = CONSTRAINTS
);

impl SessionPassword {
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
