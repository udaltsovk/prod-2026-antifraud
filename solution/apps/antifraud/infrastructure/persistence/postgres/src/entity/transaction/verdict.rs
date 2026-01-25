use domain::transaction::status::TransactionStatus;
use lib::model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = TransactionStatus, from, into)]
#[sqlx(type_name = "transaction_verdict", rename_all = "UPPERCASE")]
pub enum StoredTransactionVerdict {
    Approved,
    Declined,
}
