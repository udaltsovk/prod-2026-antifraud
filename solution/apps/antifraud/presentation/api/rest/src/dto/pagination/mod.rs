use domain::pagination::PaginationInput;
use lib::presentation::api::rest::{
    into_validators,
    validation::{
        LossyUserInput, parseable::Parseable, validator::ValidatorResult,
    },
};
use serde::{Deserialize, Serialize};

mod time_based;
pub use time_based::TimeBasedPaginationQuery;

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
        PaginationInput {
            page,
            size,
        }: PaginationInput,
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
pub struct PaginationQuery {
    #[serde(default)]
    pub page: LossyUserInput<i64>,

    #[serde(default)]
    pub size: LossyUserInput<i64>,
}

impl Parseable<PaginationInput> for PaginationQuery {
    fn parse(self) -> ValidatorResult<PaginationInput> {
        let (errors, (page, size)) = into_validators!(
            field!(self.page, optional, "page"),
            field!(self.size, optional, "size")
        );

        errors.into_result(|ok| PaginationInput {
            page: page.validated(ok),
            size: size.validated(ok),
        })
    }
}
