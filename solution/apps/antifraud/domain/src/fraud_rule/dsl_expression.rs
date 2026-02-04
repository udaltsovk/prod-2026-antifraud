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
pub struct FraudRuleDslExpression(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(constraints::length::Min(3))
        .add_constraint(constraints::length::Max(2000))
        .build()
});

impl TryFrom<String> for FraudRuleDslExpression {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = FraudRuleDslExpression,
    input_type = String
);
