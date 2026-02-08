use domain::transaction::filter::TransactionFilterInput;
use lib::{
    presentation::api::rest::{
        into_validators,
        validation::{
            LossyUserInput, parseable::Parseable, validator::ValidatorResult,
        },
    },
    uuid::Uuid,
};
use serde::Deserialize;

use crate::dto::pagination::{PaginationQuery, TimeBasedPaginationQuery};

#[derive(Deserialize, Clone, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct TransactionFilterQuery {
    #[serde(default)]
    pub user_id: LossyUserInput<Uuid>,

    #[serde(default)]
    pub status: LossyUserInput<String>,

    #[serde(default)]
    pub is_fraud: LossyUserInput<bool>,

    #[serde(default, flatten)]
    pub time_based_pagination: TimeBasedPaginationQuery,

    #[serde(default, flatten)]
    pub pagination: PaginationQuery,
}

impl Parseable<TransactionFilterInput> for TransactionFilterQuery {
    fn parse(self) -> ValidatorResult<TransactionFilterInput> {
        let (errors, (user_id, status, time_based_pagination, pagination)) = into_validators!(
            field!(self.user_id, optional, "userId"),
            field!(self.status, optional, "status"),
            field!(self.time_based_pagination, nested, None),
            field!(self.pagination, nested, None),
        );

        errors.into_result(|ok| TransactionFilterInput {
            user_id: user_id.validated(ok),
            status: status.validated(ok),
            time_based_pagination: time_based_pagination.validated(ok),
            pagination: pagination.validated(ok),
        })
    }
}
