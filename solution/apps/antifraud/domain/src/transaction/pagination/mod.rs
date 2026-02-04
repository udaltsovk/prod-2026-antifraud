use chrono::{DateTime, Days, Utc};
use lib::domain::{DomainType, Id};

use crate::{
    pagination::Pagination,
    transaction::{
        pagination::{
            from::TransactionPaginationFrom, to::TransactionPaginationTo,
        },
        status::TransactionStatus,
        user_id::TransactionUserId,
    },
    user::{User, role::UserRole},
};

pub mod constraints;
pub mod from;
pub mod to;

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionPagination {
    pub user_id: Option<TransactionUserId>,
    pub status: Option<TransactionStatus>,
    pub from: Option<TransactionPaginationFrom>,
    pub to: Option<TransactionPaginationTo>,
    pub pagination: Pagination,
}

// TODO: fix that lint
#[expect(clippy::type_complexity, reason = "I'll fix that later")]
impl TransactionPagination {
    #[must_use]
    pub fn into_parts(
        self,
        (requester_id, requester_role): (Id<User>, UserRole),
    ) -> (
        Option<Id<User>>,
        Option<TransactionStatus>,
        DateTime<Utc>,
        DateTime<Utc>,
        i64,
        i64,
    ) {
        let user_id = self
            .user_id
            .map(DomainType::into_inner)
            .map(Id::from)
            .or_else(|| {
                requester_role.ne(&UserRole::Admin).then_some(requester_id)
            });

        let status = self.status;
        let to = self.to.unwrap_or_default().into_inner();
        let from = self.from.map_or_else(
            || {
                to.checked_sub_days(Days::new(90))
                    .unwrap_or(DateTime::<Utc>::MIN_UTC)
            },
            DomainType::into_inner,
        );
        let (limit, offset) = self.pagination.into_limit_offset();

        (user_id, status, from, to, limit, offset)
    }
}
