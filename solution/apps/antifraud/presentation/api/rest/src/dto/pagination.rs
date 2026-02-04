use domain::pagination::Pagination;
use lib::presentation::api::rest::{
    into_validators,
    validation::{
        LossyUserInput, parseable::Parseable, validator::ValidatorResult,
    },
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
    size: u8,
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

#[derive(Deserialize, Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct QueryPagination {
    #[serde(default)]
    pub page: LossyUserInput<i64>,

    #[serde(default)]
    pub size: LossyUserInput<i64>,
}

impl Parseable<Pagination> for QueryPagination {
    fn parse(self) -> ValidatorResult<Pagination> {
        let (errors, (page, size)) = into_validators!(
            field!(self.page.0, optional, "page"),
            field!(self.size.0, optional, "size")
        );

        errors.into_result(|ok| Pagination {
            page: page.validated(ok),
            size: size.validated(ok),
        })
    }
}
