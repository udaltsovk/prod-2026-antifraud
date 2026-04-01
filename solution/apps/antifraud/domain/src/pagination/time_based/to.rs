use std::sync::LazyLock;

use chrono::{DateTime, Utc};
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

#[derive(DomainType, Clone, Debug)]
pub struct TimeBasedPaginationTo(DateTime<Utc>);

static CONSTRAINTS: LazyLock<Constraints<DateTime<Utc>>> =
    LazyLock::new(|| Constraints::builder().build());

impl TryFrom<DateTime<Utc>> for TimeBasedPaginationTo {
    type Error = ValidationErrors;

    fn try_from(value: DateTime<Utc>) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TimeBasedPaginationTo,
    input_type = DateTime<Utc>
);

impl Default for TimeBasedPaginationTo {
    fn default() -> Self {
        Self(Utc::now())
    }
}
