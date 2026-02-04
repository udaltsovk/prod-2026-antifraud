use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        impl_try_from_external_input,
        validation::{
            Constraints,
            constraints::{self, Constraint},
            error::{ValidationErrors, ValidationResult},
        },
    },
};

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionLocationCountry(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(constraints::length::Min(2))
        .add_constraint(constraints::length::Max(2))
        .add_constraint(IsUppercase)
        .add_constraint(IsIso3166Alpha2CountryCode)
        .build()
});

impl TryFrom<String> for TransactionLocationCountry {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionLocationCountry,
    input_type = String
);

struct IsUppercase;

impl Constraint<String> for IsUppercase {
    fn check(&self, value: &String) -> bool {
        value.chars().all(char::is_uppercase)
    }

    fn error_msg(&self) -> String {
        "must be uppercase".into()
    }
}

struct IsIso3166Alpha2CountryCode;

impl Constraint<String> for IsIso3166Alpha2CountryCode {
    fn check(&self, value: &String) -> bool {
        rust_iso3166::from_alpha2(value).is_some()
    }

    fn error_msg(&self) -> String {
        "must a valid ISO 3166-1 alpha-2 country code".into()
    }
}
