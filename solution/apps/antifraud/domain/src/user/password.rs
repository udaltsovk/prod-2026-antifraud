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
    redact::Secret,
};

use crate::{constraints::PASSWORD_LENGTH_CONSTRAINTS, password::Password};

#[derive(DomainType, PartialEq, Eq, Clone, Debug)]
pub struct UserPassword(Secret<String>);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with(&PASSWORD_LENGTH_CONSTRAINTS)
        .add_constraint(
            constraints::has::Letter::with_err(|_| {
                "must contain at least one letter".into()
            })
            .build(),
        )
        .add_constraint(
            constraints::has::Digit::with_err(|_| {
                "must contain at least one digit".to_string()
            })
            .build(),
        )
        .build()
});

impl TryFrom<Secret<String>> for UserPassword {
    type Error = ValidationErrors;

    fn try_from(value: Secret<String>) -> ValidationResult<Self> {
        CONSTRAINTS
            .check(value.expose_secret())
            .into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = UserPassword,
    input_type = Secret<String>
);

impl From<UserPassword> for Password {
    fn from(password: UserPassword) -> Self {
        Self(password.0)
    }
}
