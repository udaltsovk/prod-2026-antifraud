use chrono::{DateTime, Days, Utc};
use domain::pagination::time_based::TimeBasedPaginationInput;
use lib::presentation::api::rest::{
    errors::validation::FieldErrors,
    into_validators,
    validation::{
        LossyUserInput, UserInput, parseable::Parseable,
        validator::ValidatorResult,
    },
};
use serde::Deserialize;

#[derive(Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeBasedPaginationQuery<const MAX_PERIOD_DAYS: u64> {
    #[serde(default)]
    pub from: LossyUserInput<DateTime<Utc>>,

    #[serde(default)]
    pub to: LossyUserInput<DateTime<Utc>>,
}

impl<const MAX_PERIOD_DAYS: u64> Parseable<TimeBasedPaginationInput>
    for TimeBasedPaginationQuery<MAX_PERIOD_DAYS>
{
    fn parse(self) -> ValidatorResult<TimeBasedPaginationInput> {
        let mut errors = match (&self.from.0, &self.to.0) {
            (UserInput::Ok(from), UserInput::Ok(to)) => {
                Self::validate_to_and_from(from, to)
            },
            (UserInput::Ok(from), _) => {
                Self::validate_to_and_from(from, &Utc::now())
            },
            (_, _) => FieldErrors::new(),
        };

        let (validation_errors, (from, to)) = into_validators!(
            field!(self.from, optional, "from"),
            field!(self.to, optional, "to")
        );

        errors.extend(validation_errors);

        errors.into_result(|ok| TimeBasedPaginationInput {
            from: from.validated(ok),
            to: to.validated(ok),
        })
    }
}

impl<const MAX_PERIOD_DAYS: u64> TimeBasedPaginationQuery<MAX_PERIOD_DAYS> {
    fn validate_to_and_from(
        from: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> FieldErrors {
        let mut errors = FieldErrors::new();

        if from >= to {
            errors.push("from", "must be less than `to`", from);
            errors.push("to", "must be greater than `from`", to);
        }

        if from
            .checked_add_days(Days::new(MAX_PERIOD_DAYS))
            .unwrap_or(DateTime::<Utc>::MAX_UTC)
            < *to
        {
            errors.push(
                "from",
                format!(
                    "must not be earlier than {MAX_PERIOD_DAYS} days from `to`"
                ),
                from,
            );
            errors.push(
                "to",
                format!(
                    "must not be later than {MAX_PERIOD_DAYS} days from `from`"
                ),
                to,
            );
        }

        errors
    }
}
