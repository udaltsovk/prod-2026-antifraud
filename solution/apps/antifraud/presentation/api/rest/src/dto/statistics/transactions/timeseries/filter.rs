use domain::statistics::transactions::filter::TransactionsTimeseriesPointFilterInput;
use lib::presentation::api::rest::{
    into_validators,
    validation::{
        LossyUserInput, parseable::Parseable, validator::ValidatorResult,
    },
};
use serde::Deserialize;

use crate::dto::pagination::TimeBasedPaginationQuery;

#[derive(Deserialize, Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct TransactionsTimeseriesPointFilterQuery {
    #[serde(default, flatten)]
    pub time_based_pagination: TimeBasedPaginationQuery,

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
        let (errors, (time_based_pagination, group_by, timezone, channel)) = into_validators!(
            field!(self.time_based_pagination, nested, None),
            field!(self.group_by, optional, "groupBy"),
            field!(self.timezone, optional, "timezone"),
            field!(self.channel, optional, "channel"),
        );

        errors.into_result(|ok| TransactionsTimeseriesPointFilterInput {
            time_based_pagination: time_based_pagination.validated(ok),
            group_by: group_by.validated(ok),
            timezone: timezone.validated(ok),
            channel: channel.validated(ok),
        })
    }
}
