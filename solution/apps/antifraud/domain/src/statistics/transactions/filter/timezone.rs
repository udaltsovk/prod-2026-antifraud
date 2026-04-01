use std::{str::FromStr as _, sync::LazyLock};

use chrono_tz::Tz;
use lib::{
    DomainType,
    domain::{
        DomainType as _, impl_try_from_external_input,
        validation::{
            Constraints,
            constraints::Constraint,
            error::{ValidationErrors, ValidationResult},
        },
    },
    tap::Pipe as _,
};

#[derive(DomainType, Clone, Debug)]
pub struct TransactionsTimeseriesPointFilterTimezone(Tz);

impl Default for TransactionsTimeseriesPointFilterTimezone {
    fn default() -> Self {
        Self(Tz::UTC)
    }
}

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(IsValidTimezone)
        .build()
});

impl TryFrom<String> for TransactionsTimeseriesPointFilterTimezone {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| {
            value
                .parse()
                .unwrap_or_else(Self::it_should_be_safe_to_unwrap())
                .pipe(Self)
        })
    }
}

impl_try_from_external_input!(
    domain_type = TransactionsTimeseriesPointFilterTimezone,
    input_type = String
);

struct IsValidTimezone;

impl Constraint<String> for IsValidTimezone {
    fn error_msg(&self, _rejected_value: &String) -> String {
        "must not be a valid timezone".to_string()
    }

    fn check(&self, value: &String) -> bool {
        Tz::from_str(value).is_ok()
    }
}
