use crate::pagination::time_based::{
    TimeBasedPagination, TimeBasedPaginationFromBase, TimeBasedPaginationInput,
};

#[derive(Clone, Debug)]
pub struct StatsOverviewFilterInput {
    pub time_based_pagination: TimeBasedPaginationInput,
}

#[derive(Clone, Copy, Debug)]
pub struct StatsOverviewFilter {
    pub time_based_pagination: TimeBasedPagination,
}

impl StatsOverviewFilterInput {
    #[must_use]
    pub fn normalize(self) -> StatsOverviewFilter {
        let time_based_pagination = self
            .time_based_pagination
            .normalize(TimeBasedPaginationFromBase::Now, 30);

        StatsOverviewFilter {
            time_based_pagination,
        }
    }
}
