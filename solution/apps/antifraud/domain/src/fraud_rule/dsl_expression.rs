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
        .add_constraint(
            constraints::length::Min::with_err(|_, len_limit| {
                format!("can't be shorter than {len_limit} characters")
            })
            .limit(3)
            .build(),
        )
        .add_constraint(
            constraints::length::Max::with_err(|_, len_limit| {
                format!("can't be longer than {len_limit} characters")
            })
            .limit(2000)
            .build(),
        )
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
