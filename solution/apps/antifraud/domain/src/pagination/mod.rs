use crate::pagination::{page::PaginationPage, size::PaginationSize};

pub mod page;
pub mod size;

#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Pagination {
    pub page: Option<PaginationPage>,
    pub size: Option<PaginationSize>,
}

impl Pagination {
    #[must_use]
    pub fn into_limit_offset(self) -> (i64, i64) {
        let limit: i64 = self.size.unwrap_or_default().into();
        let offset = limit.saturating_mul(self.page.unwrap_or_default().into());
        (limit, offset)
    }
}
