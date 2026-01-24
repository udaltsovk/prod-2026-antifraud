use std::{fmt, sync::LazyLock};

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

#[derive(DomainType, Debug, PartialEq, Eq)]
pub struct FraudRuleName(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("name")
        .add_constraint(constraints::length::Min(3))
        .add_constraint(constraints::length::Max(120))
        .build()
});

impl TryFrom<String> for FraudRuleName {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = FraudRuleName,
    input_type = String,
    constraints = CONSTRAINTS
);

impl fmt::Display for FraudRuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
