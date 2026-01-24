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

#[derive(DomainType, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserFullName(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("fullName")
        .add_constraint(constraints::length::Min(2))
        .add_constraint(constraints::length::Max(200))
        .build()
});

impl TryFrom<String> for UserFullName {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = UserFullName,
    input_type = String,
    constraints = CONSTRAINTS
);
