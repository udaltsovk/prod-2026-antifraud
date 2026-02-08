use domain::statistics::overview::filter::StatsOverviewFilterInput;
use lib::presentation::api::rest::{
    into_validators,
    validation::{parseable::Parseable, validator::ValidatorResult},
};
use serde::Deserialize;

use crate::dto::pagination::TimeBasedPaginationQuery;

#[derive(Deserialize, Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct StatsOverviewFilterQuery {
    #[serde(default, flatten)]
    pub time_based_pagination: TimeBasedPaginationQuery,
}

impl Parseable<StatsOverviewFilterInput> for StatsOverviewFilterQuery {
    fn parse(self) -> ValidatorResult<StatsOverviewFilterInput> {
        let (errors, time_based_pagination) =
            into_validators!(field!(self.time_based_pagination, nested, None),);

        errors.into_result(|ok| StatsOverviewFilterInput {
            time_based_pagination: time_based_pagination.validated(ok),
        })
    }
}
