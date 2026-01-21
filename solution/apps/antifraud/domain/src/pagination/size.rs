use std::{fmt::Debug, num::NonZero, sync::LazyLock};

use lib::{
    DomainType,
    domain::{
        DomainType as _,
        validation::{
            Constraints, constraints,
            error::{ValidationErrors, ValidationResult},
        },
    },
    tap::{Pipe as _, TryConv as _},
};

#[derive(DomainType, Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PaginationSize(NonZero<u8>);

static CONSTRAINTS: LazyLock<Constraints<i64>> = LazyLock::new(|| {
    Constraints::builder("size")
        .add_constraint(constraints::range::Min(1_i64))
        .add_constraint(constraints::range::Max(100_i64))
        .build()
});
impl TryFrom<i64> for PaginationSize {
    type Error = ValidationErrors;

    fn try_from(value: i64) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| {
            value
                .try_conv::<u8>()
                .map_or_else(
                    Self::it_should_be_safe_to_unwrap(CONSTRAINTS.name()),
                    |val| {
                        NonZero::new(val).unwrap_or_else(|| {
                            Self::it_should_be_safe_to_unwrap(
                                CONSTRAINTS.name(),
                            )(())
                        })
                    },
                )
                .pipe(Self)
        })
    }
}

impl From<PaginationSize> for i64 {
    fn from(age: PaginationSize) -> Self {
        age.0.get().into()
    }
}

impl Default for PaginationSize {
    fn default() -> Self {
        Self(
            NonZero::<u8>::try_from(20)
                .expect("20 is not zero is in the u8 range"),
        )
    }
}
