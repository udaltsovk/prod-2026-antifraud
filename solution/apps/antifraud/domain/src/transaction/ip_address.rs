use std::{net::IpAddr, sync::LazyLock};

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
pub struct TransactionIpAddress(IpAddr);

static CONSTRAINTS: LazyLock<Constraints<IpAddr>> =
    LazyLock::new(|| Constraints::builder().build());

impl TryFrom<IpAddr> for TransactionIpAddress {
    type Error = ValidationErrors;

    fn try_from(value: IpAddr) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionIpAddress,
    input_type = IpAddr
);
