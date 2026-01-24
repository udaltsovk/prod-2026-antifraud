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
pub struct TransactionPaginationFrom(DateTime<Utc>);

static CONSTRAINTS: LazyLock<Constraints<DateTime<Utc>>> =
    LazyLock::new(|| Constraints::builder("from").build());

impl TryFrom<DateTime<Utc>> for TransactionPaginationFrom {
    type Error = ValidationErrors;

    fn try_from(value: DateTime<Utc>) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionPaginationFrom,
    input_type = DateTime<Utc>,
    constraints = CONSTRAINTS
);
