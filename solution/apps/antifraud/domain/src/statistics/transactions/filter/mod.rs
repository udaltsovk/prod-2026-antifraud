use crate::{
    pagination::time_based::{
        TimeBasedPagination, TimeBasedPaginationFromBase,
        TimeBasedPaginationInput,
    },
    statistics::transactions::filter::{
        group_by::TransactionsTimeseriesPointFilterGroupBy,
        timezone::TransactionsTimeseriesPointFilterTimezone,
    },
    transaction::channel::TransactionChannel,
};

pub mod group_by;
pub mod timezone;

#[derive(Clone, Debug)]
pub struct TransactionsTimeseriesPointFilterInput {
    pub time_based_pagination: TimeBasedPaginationInput,
    pub group_by: Option<TransactionsTimeseriesPointFilterGroupBy>,
    pub timezone: Option<TransactionsTimeseriesPointFilterTimezone>,
    pub channel: Option<TransactionChannel>,
}

#[derive(Clone, Debug)]
pub struct TransactionsTimeseriesPointFilter {
    pub time_based_pagination: TimeBasedPagination,
    pub group_by: TransactionsTimeseriesPointFilterGroupBy,
    pub timezone: TransactionsTimeseriesPointFilterTimezone,
    pub channel: Option<TransactionChannel>,
}

impl TransactionsTimeseriesPointFilterInput {
    #[must_use]
    pub fn normalize(self) -> TransactionsTimeseriesPointFilter {
        let time_based_pagination = self
            .time_based_pagination
            .normalize(TimeBasedPaginationFromBase::Now, 7);

        TransactionsTimeseriesPointFilter {
            time_based_pagination,
            group_by: self.group_by.unwrap_or_default(),
            timezone: self.timezone.unwrap_or_default(),
            channel: self.channel,
        }
    }
}
