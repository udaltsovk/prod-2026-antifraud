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

#[derive(DomainType, Clone, Debug, PartialEq, Eq)]
pub struct FraudRuleName(String);

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
            .limit(120)
            .build(),
        )
        .build()
});

impl TryFrom<String> for FraudRuleName {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(domain_type = FraudRuleName, input_type = String);

impl fmt::Display for FraudRuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
