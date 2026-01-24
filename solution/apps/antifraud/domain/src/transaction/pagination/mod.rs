use chrono::{DateTime, Days, Utc};
use lib::domain::{DomainType, Id, validation::Optional};

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
    pub user_id: Optional<TransactionUserId>,
    pub status: Optional<TransactionStatus>,
    pub from: Optional<TransactionPaginationFrom>,
    pub to: Optional<TransactionPaginationTo>,
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
            .into_option()
            .or_else(|| {
                requester_role.ne(&UserRole::Admin).then_some(requester_id)
            });

        let status = self.status.into_option();
        let to = self.to.unwrap_or_default().into_inner();
        let from = self.from.map(DomainType::into_inner).unwrap_or_else(|| {
            to.checked_sub_days(Days::new(90))
                .unwrap_or(DateTime::<Utc>::MIN_UTC)
        });
        let (limit, offset) = self.pagination.into_limit_offset();

        (user_id, status, to, from, limit, offset)
    }
}
