use domain::transaction::channel::TransactionChannel;
use lib::model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = TransactionChannel, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionChannelDto {
    Web,
    Mobile,
    Pos,
    Other,
}
