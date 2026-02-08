use crate::{
    pagination::time_based::{
        TimeBasedPagination, TimeBasedPaginationFromBase,
        TimeBasedPaginationInput,
    },
    statistics::merchants::filter::top::MerchantRiskStatsFilterTop,
    transaction::merchant_category_code::TransactionMerchantCategoryCode,
};

pub mod top;

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MerchantsRiskStatsFilterInput {
    pub time_based_pagination: TimeBasedPaginationInput,
    pub merchant_category_code: Option<TransactionMerchantCategoryCode>,
    pub top: Option<MerchantRiskStatsFilterTop>,
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MerchantsRiskStatsFilter {
    pub time_based_pagination: TimeBasedPagination,
    pub merchant_category_code: Option<TransactionMerchantCategoryCode>,
    pub top: u8,
}

impl MerchantsRiskStatsFilterInput {
    #[must_use]
    pub fn normalize(self) -> MerchantsRiskStatsFilter {
        let time_based_pagination = self
            .time_based_pagination
            .normalize(TimeBasedPaginationFromBase::Now, 30);

        MerchantsRiskStatsFilter {
            time_based_pagination,
            merchant_category_code: self.merchant_category_code,
            top: 50,
        }
    }
}
