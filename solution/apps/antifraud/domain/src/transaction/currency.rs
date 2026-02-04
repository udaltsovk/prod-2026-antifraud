use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        impl_try_from_external_input,
        validation::{
            Constraints,
            constraints::{self, regex::Regex},
            error::{ValidationErrors, ValidationResult},
        },
    },
};

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionCurrency(String);

static TRANSACTION_CURRENCY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^[A-Z]{3}$").expect("that regex should be valid")
});

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(constraints::Matches(
            TRANSACTION_CURRENCY_REGEX.clone(),
        ))
        .build()
});

impl TryFrom<String> for TransactionCurrency {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionCurrency,
    input_type = String
);
