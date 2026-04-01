use domain::transaction::status::TransactionStatus;
use model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize, Debug)]
#[mapper(ty = TransactionStatus, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionStatusDto {
    Approved,
    Declined,
}
