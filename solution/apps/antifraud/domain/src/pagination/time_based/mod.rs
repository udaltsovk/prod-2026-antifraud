use chrono::{DateTime, Days, Utc};
use lib::domain::DomainType;

use crate::pagination::time_based::{
    from::TimeBasedPaginationFrom, to::TimeBasedPaginationTo,
};

pub mod from;
pub mod to;

#[derive(Clone, Debug)]
pub struct TimeBasedPaginationInput {
    pub from: Option<TimeBasedPaginationFrom>,
    pub to: Option<TimeBasedPaginationTo>,
}

#[derive(Clone, Copy, Debug)]
pub struct TimeBasedPagination {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug)]
pub enum TimeBasedPaginationFromBase {
    Now,
    To,
}

impl TimeBasedPaginationInput {
    #[must_use]
    pub fn normalize(
        self,
        from_base: TimeBasedPaginationFromBase,
        default_from_offset_days: u8,
    ) -> TimeBasedPagination {
        let to = self.to.unwrap_or_default().into_inner();
        let from = self.from.map_or_else(
            || {
                match from_base {
                    TimeBasedPaginationFromBase::Now => Utc::now(),
                    TimeBasedPaginationFromBase::To => to,
                }
                .checked_sub_days(Days::new(default_from_offset_days.into()))
                .unwrap_or(DateTime::<Utc>::MIN_UTC)
            },
            DomainType::into_inner,
        );

        TimeBasedPagination {
            from,
            to,
        }
    }
}
