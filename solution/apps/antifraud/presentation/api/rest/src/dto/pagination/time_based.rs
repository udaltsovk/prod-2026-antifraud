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

#[derive(Deserialize, Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct TimeBasedPaginationQuery {
    #[serde(default)]
    pub from: LossyUserInput<DateTime<Utc>>,

    #[serde(default)]
    pub to: LossyUserInput<DateTime<Utc>>,
}

impl Parseable<TimeBasedPaginationInput> for TimeBasedPaginationQuery {
    fn parse(self) -> ValidatorResult<TimeBasedPaginationInput> {
        let mut errors = match (&self.from, &self.to) {
            (
                LossyUserInput(UserInput::Ok(from)),
                LossyUserInput(UserInput::Ok(to)),
            ) => validate_to_and_from(from, to),
            (LossyUserInput(UserInput::Ok(from)), _) => {
                validate_to_and_from(from, &Utc::now())
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
        .checked_add_days(Days::new(90))
        .unwrap_or(DateTime::<Utc>::MAX_UTC)
        < *to
    {
        errors.push("from", "must not be earlier than 90 days from `to`", from);
        errors.push("to", "must not be later than 90 days from `from`", to);
    }

    errors
}
