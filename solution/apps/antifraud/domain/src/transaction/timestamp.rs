use std::sync::LazyLock;

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use lib::{
    DomainType,
    domain::{
        impl_try_from_external_input,
        validation::{
            Constraints,
            constraints::Constraint,
            error::{ValidationErrors, ValidationResult},
        },
    },
};

#[derive(DomainType, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionTimestamp(DateTime<Utc>);

static CONSTRAINTS: LazyLock<Constraints<DateTime<Utc>>> =
    LazyLock::new(|| {
        Constraints::builder()
            .add_constraint(TimedeltaLessThan(TimeDelta::minutes(5)))
            .build()
    });

impl TryFrom<DateTime<Utc>> for TransactionTimestamp {
    type Error = ValidationErrors;

    fn try_from(value: DateTime<Utc>) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = TransactionTimestamp,
    input_type = DateTime<Utc>
);

struct TimedeltaLessThan(TimeDelta);

impl<Tz> Constraint<DateTime<Tz>> for TimedeltaLessThan
where
    Tz: TimeZone,
{
    fn error_msg(&self) -> String {
        format!(
            "must not be further in the future by more than `{}`",
            self.0
        )
    }

    fn check(&self, value: &DateTime<Tz>) -> bool {
        let timezone = &value.timezone();
        Utc::now()
            .with_timezone(timezone)
            .checked_add_signed(self.0)
            .unwrap_or_else(|| DateTime::<Tz>::MAX_UTC.with_timezone(timezone))
            .gt(value)
    }
}
