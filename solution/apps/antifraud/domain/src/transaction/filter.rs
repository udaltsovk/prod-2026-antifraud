use lib::domain::{DomainType, Id};

use crate::{
    pagination::{
        Pagination, PaginationInput,
        time_based::{
            TimeBasedPagination, TimeBasedPaginationFromBase,
            TimeBasedPaginationInput,
        },
    },
    transaction::{status::TransactionStatus, user_id::TransactionUserId},
    user::{User, role::UserRole},
};

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionFilterInput {
    pub user_id: Option<TransactionUserId>,
    pub status: Option<TransactionStatus>,
    pub time_based_pagination: TimeBasedPaginationInput,
    pub pagination: PaginationInput,
}

#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionFilter {
    pub requester_id: Option<Id<User>>,
    pub status: Option<TransactionStatus>,
    pub time_based_pagination: TimeBasedPagination,
    pub pagination: Pagination,
}

impl TransactionFilterInput {
    #[must_use]
    pub fn normalize(
        self,
        (requester_id, requester_role): (Id<User>, UserRole),
    ) -> TransactionFilter {
        let user_id = self
            .user_id
            .map(DomainType::into_inner)
            .map(Id::from)
            .or_else(|| {
                requester_role.ne(&UserRole::Admin).then_some(requester_id)
            });

        let status = self.status;
        let time_based_pagination = self
            .time_based_pagination
            .normalize(TimeBasedPaginationFromBase::To, 90);
        let pagination = self.pagination.normalize();

        TransactionFilter {
            requester_id: user_id,
            status,
            time_based_pagination,
            pagination,
        }
    }
}
