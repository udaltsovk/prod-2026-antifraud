use crate::pagination::{page::PaginationPage, size::PaginationSize};

pub mod page;
pub mod size;
pub mod time_based;

#[derive(Clone, Copy, Debug)]
pub struct PaginationInput {
    pub page: Option<PaginationPage>,
    pub size: Option<PaginationSize>,
}

#[derive(Clone, Copy, Debug)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}

impl PaginationInput {
    #[must_use]
    pub fn normalize(self) -> Pagination {
        let limit: i64 = self.size.unwrap_or_default().into();
        let offset = limit.saturating_mul(self.page.unwrap_or_default().into());

        Pagination {
            limit,
            offset,
        }
    }
}
