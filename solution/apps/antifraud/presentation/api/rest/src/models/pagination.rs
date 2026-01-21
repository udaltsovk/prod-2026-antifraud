use std::num::NonZero;

use domain::pagination::Pagination;
use lib::{
    domain::{into_option_validators, validation::error::ValidationResult},
    presentation::api::rest::model::Parseable,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Paginated<T>
where
    T: Serialize,
{
    items: Vec<T>,
    total: u64,
    page: u64,
    size: NonZero<u8>,
}

impl<T> Paginated<T>
where
    T: Serialize,
{
    pub fn from_pagination<I>(
        Pagination {
            page,
            size,
        }: Pagination,
        items: Vec<I>,
        total: u64,
    ) -> Self
    where
        I: Into<T>,
    {
        Self {
            items: items.into_iter().map(I::into).collect(),
            total,
            page: page.unwrap_or_default().into(),
            size: size.unwrap_or_default().into(),
        }
    }
}

#[derive(Deserialize, Clone, Copy, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct QueryPagination {
    pub page: Option<i64>,
    pub size: Option<i64>,
}

impl Parseable<Pagination> for QueryPagination {
    const FIELD: &str = "pagination";

    fn parse(self) -> ValidationResult<Pagination> {
        let (errors, (page, size)) =
            into_option_validators!(self.page, self.size);

        errors.into_result(|ok| Pagination {
            page: page.validated(ok),
            size: size.validated(ok),
        })
    }
}
