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

#[derive(DomainType, Clone, Copy, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionAmount(f64);

static CONSTRAINTS: LazyLock<Constraints<f64>> = LazyLock::new(|| {
    Constraints::builder("amount")
        .add_constraint(constraints::range::Min(0.01_f64))
        .add_constraint(constraints::range::Max(9_999_999_999.99_f64))
        .build()
});

impl TryFrom<f64> for TransactionAmount {
    type Error = ValidationErrors;

    fn try_from(value: f64) -> ValidationResult<Self> {
        let value = (value * 100.0).round() / 100.0_f64;
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionAmount,
    input_type = f64,
    constraints = CONSTRAINTS
);
