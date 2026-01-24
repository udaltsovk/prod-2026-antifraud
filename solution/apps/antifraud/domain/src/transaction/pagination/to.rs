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

#[derive(DomainType, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionPaginationTo(DateTime<Utc>);

static CONSTRAINTS: LazyLock<Constraints<DateTime<Utc>>> =
    LazyLock::new(|| Constraints::builder("to").build());

impl TryFrom<DateTime<Utc>> for TransactionPaginationTo {
    type Error = ValidationErrors;

    fn try_from(value: DateTime<Utc>) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionPaginationTo,
    input_type = DateTime<Utc>,
    constraints = CONSTRAINTS
);

impl Default for TransactionPaginationTo {
    fn default() -> Self {
        Self(Utc::now())
    }
}
