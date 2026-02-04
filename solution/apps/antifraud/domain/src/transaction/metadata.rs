use std::sync::LazyLock;

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
use serde_value::Value;

#[derive(DomainType, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionMetadata(Value);

static CONSTRAINTS: LazyLock<Constraints<Value>> =
    LazyLock::new(|| Constraints::builder().build());

impl TryFrom<Value> for TransactionMetadata {
    type Error = ValidationErrors;

    fn try_from(value: Value) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionMetadata,
    input_type = Value
);

#[doc(hidden)]
impl TryFrom<serde_json::Value> for TransactionMetadata {
    type Error = ValidationErrors;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_value::to_value(&value)
            .map_err(|issue| ValidationErrors::with_error(issue, value))
            .map(Self)
    }
}

#[doc(hidden)]
impl From<TransactionMetadata> for serde_json::Value {
    fn from(value: TransactionMetadata) -> Self {
        serde_json::to_value(value.0).unwrap_or_default()
    }
}
