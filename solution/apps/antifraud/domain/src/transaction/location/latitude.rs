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

#[derive(DomainType, Clone, Copy, Default, Debug)]
pub struct TransactionLocationLatitude(f32);

static CONSTRAINTS: LazyLock<Constraints<f32>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(
            constraints::range::Min::with_err(|_, limit| {
                format!("can't be less than {limit}")
            })
            .limit(-90_f32)
            .build(),
        )
        .add_constraint(
            constraints::range::Max::with_err(|_, limit| {
                format!("can't be greater than {limit}")
            })
            .limit(90_f32)
            .build(),
        )
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
    input_type = f32
);
