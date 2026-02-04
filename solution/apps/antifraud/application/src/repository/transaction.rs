use chrono::{DateTime, Utc};
use domain::{
    transaction::{Transaction, status::TransactionStatus},
    user::User,
};
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

    async fn list(
        &self,
        requester_id: Option<Id<User>>,
        status: Option<TransactionStatus>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Transaction>>;

    async fn count(
        &self,
        requester_id: Option<Id<User>>,
        status: Option<TransactionStatus>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<i64>;
}
