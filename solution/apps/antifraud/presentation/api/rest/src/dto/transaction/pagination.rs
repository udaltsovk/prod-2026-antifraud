use chrono::{DateTime, Days, Utc};
use domain::{
    pagination::Pagination, transaction::pagination::TransactionPagination,
};
use lib::{
    presentation::api::rest::{
        errors::validation::FieldErrors,
        into_validators,
        validation::{
            LossyUserInput, UserInput, parseable::Parseable,
            validator::ValidatorResult,
        },
    },
    uuid::Uuid,
};
use serde::Deserialize;

#[derive(Deserialize, Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct QueryTransactionPagination {
    #[serde(default)]
    pub user_id: LossyUserInput<Uuid>,

    #[serde(default)]
    pub status: LossyUserInput<String>,

    #[serde(default)]
    pub is_fraud: LossyUserInput<bool>,

    #[serde(default)]
    pub from: LossyUserInput<DateTime<Utc>>,

    #[serde(default)]
    pub to: LossyUserInput<DateTime<Utc>>,

    #[serde(default)]
    pub page: LossyUserInput<i64>,

    #[serde(default)]
    pub size: LossyUserInput<i64>,
}

impl Parseable<TransactionPagination> for QueryTransactionPagination {
    fn parse(self) -> ValidatorResult<TransactionPagination> {
        let (mut errors, (user_id, status, page, size)) = into_validators!(
            field!(self.user_id.0, optional, "userId"),
            field!(self.status.0, optional, "status"),
            field!(self.page.0, optional, "page"),
            field!(self.size.0, optional, "size")
        );

        let time_errors = match (&self.from, &self.to) {
            (
                LossyUserInput(UserInput::Ok(from)),
                LossyUserInput(UserInput::Ok(to)),
            ) => validate_to_and_from(from, to),
            (LossyUserInput(UserInput::Ok(from)), _) => {
                validate_to_and_from(from, &Utc::now())
            },
            (_, _) => FieldErrors::new(),
        };

        let (time_validation_errors, (from, to)) = into_validators!(
            field!(self.from.0, optional, "from"),
            field!(self.to.0, optional, "to")
        );

        errors.extend(time_validation_errors);
        errors.extend(time_errors);

        errors.into_result(|ok| TransactionPagination {
            user_id: user_id.validated(ok),
            status: status.validated(ok),
            from: from.validated(ok),
            to: to.validated(ok),
            pagination: Pagination {
                page: page.validated(ok),
                size: size.validated(ok),
            },
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
