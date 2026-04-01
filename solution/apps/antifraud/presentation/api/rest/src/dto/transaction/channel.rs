use domain::transaction::channel::TransactionChannel;
use model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize, Debug)]
#[mapper(ty = TransactionChannel, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionChannelDto {
    Web,
    Mobile,
    Pos,
    Other,
}
