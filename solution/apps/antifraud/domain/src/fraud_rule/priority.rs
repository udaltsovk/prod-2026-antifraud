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

#[derive(DomainType, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FraudRulePriority(i64);

static CONSTRAINTS: LazyLock<Constraints<i64>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(
            constraints::range::Min::with_err(|_, limit| {
                format!("can't be less than {limit}")
            })
            .limit(1_i64)
            .build(),
        )
        .add_constraint(
            constraints::range::Max::with_err(|_, limit| {
                format!("can't be greater than {limit}")
            })
            .limit(i64::MAX)
            .build(),
        )
        .build()
});
impl TryFrom<i64> for FraudRulePriority {
    type Error = ValidationErrors;

    fn try_from(value: i64) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = FraudRulePriority,
    input_type = i64
);

impl Default for FraudRulePriority {
    fn default() -> Self {
        Self(100)
    }
}
