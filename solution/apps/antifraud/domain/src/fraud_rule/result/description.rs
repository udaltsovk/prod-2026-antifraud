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

#[derive(DomainType, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FraudRuleResultDescription(pub String);

static CONSTRAINTS: LazyLock<Constraints<String>> =
    LazyLock::new(|| Constraints::builder("description").build());

impl TryFrom<String> for FraudRuleResultDescription {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = FraudRuleResultDescription,
    input_type = String,
    constraints = CONSTRAINTS
);
