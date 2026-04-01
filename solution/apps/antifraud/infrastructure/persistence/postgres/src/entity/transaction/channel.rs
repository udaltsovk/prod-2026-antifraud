use domain::transaction::channel::TransactionChannel;
use model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type, Debug)]
#[mapper(ty = TransactionChannel, from, into)]
#[sqlx(type_name = "transaction_channel", rename_all = "UPPERCASE")]
pub enum StoredTransactionChannel {
    Web,
    Mobile,
    Pos,
    Other,
}
