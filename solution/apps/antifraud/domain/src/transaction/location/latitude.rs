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
pub struct TransactionLocationLatitude(f32);

static CONSTRAINTS: LazyLock<Constraints<f32>> = LazyLock::new(|| {
    Constraints::builder("latitude")
        .add_constraint(constraints::range::Min(-90_f32))
        .add_constraint(constraints::range::Max(90_f32))
        .build()
});

impl TryFrom<f32> for TransactionLocationLatitude {
    type Error = ValidationErrors;

    fn try_from(value: f32) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionLocationLatitude,
    input_type = f32,
    constraints = CONSTRAINTS
);
