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
pub struct PaginationPage(i64);

static CONSTRAINTS: LazyLock<Constraints<i64>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(constraints::range::Min(0_i64))
        .build()
});

impl TryFrom<i64> for PaginationPage {
    type Error = ValidationErrors;

    fn try_from(value: i64) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(domain_type = PaginationPage, input_type = i64);

impl From<PaginationPage> for u64 {
    fn from(value: PaginationPage) -> Self {
        value.0.try_into().unwrap_or(Self::MIN)
    }
}
