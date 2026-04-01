use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        DomainType as _, impl_try_from_external_input,
        validation::{
            Constraints, constraints,
            error::{ValidationErrors, ValidationResult},
        },
    },
    tap::{Pipe as _, TryConv as _},
};

#[derive(DomainType, Clone, Copy, Debug)]
pub struct PaginationSize(u8);

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
            .limit(100_i64)
            .build(),
        )
        .build()
});
impl TryFrom<i64> for PaginationSize {
    type Error = ValidationErrors;

    fn try_from(value: i64) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| {
            value
                .try_conv::<u8>()
                .unwrap_or_else(Self::it_should_be_safe_to_unwrap())
                .pipe(Self)
        })
    }
}

impl_try_from_external_input!(domain_type = PaginationSize, input_type = i64);

impl From<PaginationSize> for i64 {
    fn from(age: PaginationSize) -> Self {
        age.0.into()
    }
}

impl Default for PaginationSize {
    fn default() -> Self {
        Self(20)
    }
}
