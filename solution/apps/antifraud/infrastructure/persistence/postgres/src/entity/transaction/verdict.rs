use domain::transaction::status::TransactionStatus;
use model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type, Debug)]
#[mapper(ty = TransactionStatus, from, into)]
#[sqlx(type_name = "transaction_verdict", rename_all = "UPPERCASE")]
pub enum StoredTransactionVerdict {
    Approved,
    Declined,
}
