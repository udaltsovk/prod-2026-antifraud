use domain::user::filter::UserFilterInput;
use lib::presentation::api::rest::{
    into_validators,
    validation::{parseable::Parseable, validator::ValidatorResult},
};
use serde::Deserialize;

use crate::dto::pagination::PaginationQuery;

#[derive(Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserFilterQuery {
    #[serde(default, flatten)]
    pub pagination: PaginationQuery,
}

impl Parseable<UserFilterInput> for UserFilterQuery {
    fn parse(self) -> ValidatorResult<UserFilterInput> {
        let (errors, pagination) =
            into_validators!(field!(self.pagination, nested, None),);

        errors.into_result(|ok| UserFilterInput {
            pagination: pagination.validated(ok),
        })
    }
}
