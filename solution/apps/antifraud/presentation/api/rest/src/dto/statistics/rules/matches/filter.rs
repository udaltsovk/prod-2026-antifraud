use domain::statistics::rules::filter::RulesMatchesStatsFilterInput;
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
pub struct RulesMatchesStatsFilterQuery {
    #[serde(default, flatten)]
    pub time_based_pagination: TimeBasedPaginationQuery<90>,

    #[serde(default)]
    pub top: LossyUserInput<i64>,
}

impl Parseable<RulesMatchesStatsFilterInput> for RulesMatchesStatsFilterQuery {
    fn parse(self) -> ValidatorResult<RulesMatchesStatsFilterInput> {
        let (errors, (time_based_pagination, top)) = into_validators!(
            field!(self.time_based_pagination, nested, None),
            field!(self.top, optional, "top")
        );

        errors.into_result(|ok| RulesMatchesStatsFilterInput {
            time_based_pagination: time_based_pagination.validated(ok),
            top: top.validated(ok),
        })
    }
}
