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

#[derive(DomainType, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionMerchantCategoryCode(String);

static TRANSCTION_MERCHANT_CATEGORY_CODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| {
        Regex::new(r"^\d{4}$").expect("that regex shoud be valid")
    });

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("merchantCategoryCode")
        .add_constraint(constraints::Matches(
            TRANSCTION_MERCHANT_CATEGORY_CODE_REGEX.clone(),
        ))
        .build()
});

impl TryFrom<String> for TransactionMerchantCategoryCode {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionMerchantCategoryCode,
    input_type = String,
    constraints = CONSTRAINTS
);
