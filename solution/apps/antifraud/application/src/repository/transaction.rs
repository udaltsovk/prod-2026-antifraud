use domain::transaction::{Transaction, filter::TransactionFilter};
use lib::{anyhow::Result, async_trait, domain::Id};

#[async_trait]
pub trait TransactionRepository {
    async fn save(&self, source: Transaction) -> Result<Transaction>;

    async fn batch_save(
        &self,
        sources: Vec<Transaction>,
    ) -> Result<Vec<Transaction>>;

    async fn find_by_id(
        &self,
        id: Id<Transaction>,
    ) -> Result<Option<Transaction>>;

    async fn list(&self, filter: TransactionFilter)
    -> Result<Vec<Transaction>>;

    async fn count(&self, filter: TransactionFilter) -> Result<i64>;
}
