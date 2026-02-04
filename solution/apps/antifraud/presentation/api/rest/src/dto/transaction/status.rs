use domain::transaction::status::TransactionStatus;
use lib::model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = TransactionStatus, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionStatusDto {
    Approved,
    Declined,
}
