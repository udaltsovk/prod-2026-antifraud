use crate::pagination::{Pagination, PaginationInput};

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserFilterInput {
    pub pagination: PaginationInput,
}

#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserFilter {
    pub pagination: Pagination,
}

impl UserFilterInput {
    #[must_use]
    pub fn normalize(self) -> UserFilter {
        let pagination = self.pagination.normalize();

        UserFilter {
            pagination,
        }
    }
}
