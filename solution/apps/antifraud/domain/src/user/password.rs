use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        impl_try_from_external_input,
        validation::{
            Constraints, constraints,
            error::{ValidationErrors, ValidationResult},
        },
    },
};

use crate::constraints::PASSWORD_LENGTH_CONSTRAINTS;

#[derive(DomainType, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserPassword(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("password", &PASSWORD_LENGTH_CONSTRAINTS)
        .add_constraint(constraints::has::Letter)
        .add_constraint(constraints::has::Digit)
        .build()
});

impl TryFrom<String> for UserPassword {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = UserPassword,
    input_type = String,
    constraints = CONSTRAINTS
);

impl UserPassword {
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
