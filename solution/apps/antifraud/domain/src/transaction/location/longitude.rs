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
pub struct TransactionLocationLongitude(f32);

static CONSTRAINTS: LazyLock<Constraints<f32>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(constraints::range::Min(-180_f32))
        .add_constraint(constraints::range::Max(180_f32))
        .build()
});

impl TryFrom<f32> for TransactionLocationLongitude {
    type Error = ValidationErrors;

    fn try_from(value: f32) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionLocationLongitude,
    input_type = f32
);
