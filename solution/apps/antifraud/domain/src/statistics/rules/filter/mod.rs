use crate::{
    pagination::time_based::{
        TimeBasedPagination, TimeBasedPaginationFromBase,
        TimeBasedPaginationInput,
    },
    statistics::rules::filter::top::RuleMatchesStatsFilterTop,
};

pub mod top;

#[derive(Clone, Debug)]
pub struct RulesMatchesStatsFilterInput {
    pub time_based_pagination: TimeBasedPaginationInput,
    pub top: Option<RuleMatchesStatsFilterTop>,
}

#[derive(Clone, Copy, Debug)]
pub struct RulesMatchesStatsFilter {
    pub time_based_pagination: TimeBasedPagination,
    pub top: u8,
}

impl RulesMatchesStatsFilterInput {
    #[must_use]
    pub fn normalize(self) -> RulesMatchesStatsFilter {
        let time_based_pagination = self
            .time_based_pagination
            .normalize(TimeBasedPaginationFromBase::Now, 30);

        RulesMatchesStatsFilter {
            time_based_pagination,
            top: 20,
        }
    }
}
