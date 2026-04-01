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

#[derive(DomainType, Debug)]
pub struct TransactionLocationCountry(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(
            constraints::length::Min::with_err(|_, len_limit| {
                format!("can't be shorter than {len_limit} characters")
            })
            .limit(2)
            .build(),
        )
        .add_constraint(
            constraints::length::Max::with_err(|_, len_limit| {
                format!("can't be longer than {len_limit} characters")
            })
            .limit(2)
            .build(),
        )
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

    fn error_msg(&self, _rejected_value: &String) -> String {
        "must be uppercase".into()
    }
}

struct IsIso3166Alpha2CountryCode;

impl Constraint<String> for IsIso3166Alpha2CountryCode {
    fn check(&self, value: &String) -> bool {
        rust_iso3166::from_alpha2(value).is_some()
    }

    fn error_msg(&self, _rejected_value: &String) -> String {
        "must a valid ISO 3166-1 alpha-2 country code".into()
    }
}
