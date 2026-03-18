use domain::statistics::transactions::filter::{
    TransactionsTimeseriesPointFilterInput,
    group_by::TransactionsTimeseriesPointFilterGroupBy,
};
use lib::presentation::api::rest::{
    into_validators,
    validation::{
        LossyUserInput, UserInput, parseable::Parseable,
        validator::ValidatorResult,
    },
};
use serde::Deserialize;

use crate::dto::pagination::TimeBasedPaginationQuery;

#[derive(Deserialize, Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct TransactionsTimeseriesPointFilterQuery {
    #[serde(default, flatten)]
    pub time_based_pagination: TimeBasedPaginationQuery<365>,

    #[serde(default)]
    pub group_by: LossyUserInput<String>,

    #[serde(default)]
    pub timezone: LossyUserInput<String>,

    #[serde(default)]
    pub channel: LossyUserInput<String>,
}

impl Parseable<TransactionsTimeseriesPointFilterInput>
    for TransactionsTimeseriesPointFilterQuery
{
    fn parse(self) -> ValidatorResult<TransactionsTimeseriesPointFilterInput> {
        let (pagination_errors, time_based_pagination) =
            match self.group_by.0.as_ref().map(|group_by| group_by.parse()) {
                UserInput::Ok(Ok(
                    TransactionsTimeseriesPointFilterGroupBy::Hour,
                )) => {
                    let TimeBasedPaginationQuery {
                        from,
                        to,
                    } = self.time_based_pagination;
                    into_validators!(field!(
                        TimeBasedPaginationQuery::<7> {
                            from,
                            to
                        },
                        nested,
                        None
                    ))
                },
                UserInput::Missing
                | UserInput::Null
                | UserInput::Ok(Ok(
                    TransactionsTimeseriesPointFilterGroupBy::Day,
                )) => {
                    let TimeBasedPaginationQuery {
                        from,
                        to,
                    } = self.time_based_pagination;
                    into_validators!(field!(
                        TimeBasedPaginationQuery::<90> {
                            from,
                            to
                        },
                        nested,
                        None
                    ))
                },
                _ => into_validators!(field!(
                    self.time_based_pagination,
                    nested,
                    None
                )),
            };

        let (mut errors, (group_by, timezone, channel)) = into_validators!(
            field!(self.group_by, optional, "groupBy"),
            field!(self.timezone, optional, "timezone"),
            field!(self.channel, optional, "channel"),
        );

        errors.extend(pagination_errors);

        errors.into_result(|ok| TransactionsTimeseriesPointFilterInput {
            time_based_pagination: time_based_pagination.validated(ok),
            group_by: group_by.validated(ok),
            timezone: timezone.validated(ok),
            channel: channel.validated(ok),
        })
    }
}
