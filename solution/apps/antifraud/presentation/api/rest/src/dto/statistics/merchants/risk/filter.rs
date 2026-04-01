use domain::statistics::merchants::filter::MerchantsRiskStatsFilterInput;
use lib::presentation::api::rest::{
    into_validators,
    validation::{
        LossyUserInput, parseable::Parseable, validator::ValidatorResult,
    },
};
use serde::Deserialize;

use crate::dto::pagination::TimeBasedPaginationQuery;

#[derive(Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerchantsRiskStatsFilterQuery {
    #[serde(default, flatten)]
    pub time_based_pagination: TimeBasedPaginationQuery<90>,

    #[serde(default)]
    pub merchant_category_code: LossyUserInput<String>,

    #[serde(default)]
    pub top: LossyUserInput<i64>,
}

impl Parseable<MerchantsRiskStatsFilterInput>
    for MerchantsRiskStatsFilterQuery
{
    fn parse(self) -> ValidatorResult<MerchantsRiskStatsFilterInput> {
        let (errors, (time_based_pagination, merchant_category_code, top)) = into_validators!(
            field!(self.time_based_pagination, nested, None),
            field!(
                self.merchant_category_code,
                optional,
                "merchantCategoryCode"
            ),
            field!(self.top, optional, "top")
        );

        errors.into_result(|ok| MerchantsRiskStatsFilterInput {
            time_based_pagination: time_based_pagination.validated(ok),
            merchant_category_code: merchant_category_code.validated(ok),
            top: top.validated(ok),
        })
    }
}
