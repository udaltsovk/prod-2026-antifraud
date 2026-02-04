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
    uuid::Uuid,
};

#[derive(DomainType, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionUserId(Uuid);

static CONSTRAINTS: LazyLock<Constraints<Uuid>> =
    LazyLock::new(|| Constraints::builder().build());

impl TryFrom<Uuid> for TransactionUserId {
    type Error = ValidationErrors;

    fn try_from(value: Uuid) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionUserId,
    input_type = Uuid
);
