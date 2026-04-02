use domain::transaction::{Transaction, filter::TransactionFilter};
use entrait::entrait;
use lib::{anyhow::Result, async_trait, domain::Id};

#[entrait(
    TransactionRepositoryImpl,
    delegate_by=DelegateTransactionRepository
)]
#[async_trait]
pub trait TransactionRepository {
    async fn save_transaction(
        &self,
        source: Transaction,
    ) -> Result<Transaction>;

    async fn batch_save_transactions(
        &self,
        sources: Vec<Transaction>,
    ) -> Result<Vec<Transaction>>;

    async fn find_transaction_by_id(
        &self,
        transaction_id: Id<Transaction>,
    ) -> Result<Option<Transaction>>;

    async fn list_transactions(
        &self,
        filter: TransactionFilter,
    ) -> Result<Vec<Transaction>>;

    async fn count_transactions(
        &self,
        filter: TransactionFilter,
    ) -> Result<i64>;
}
