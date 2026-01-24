use std::fmt::Debug;

use chrono::{DateTime, Utc};
use domain::{
    transaction::{Transaction, status::TransactionStatus},
    user::User,
};
use lib::{async_trait, domain::Id};

#[async_trait]
pub trait TransactionRepository {
    type AdapterError: Debug + Send + Sync;

    async fn save(
        &self,
        source: Transaction,
    ) -> Result<Transaction, Self::AdapterError>;

    async fn batch_save(
        &self,
        sources: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, Self::AdapterError>;

    async fn find_by_id(
        &self,
        id: Id<Transaction>,
    ) -> Result<Option<Transaction>, Self::AdapterError>;

    async fn list(
        &self,
        requester_id: Option<Id<User>>,
        status: Option<TransactionStatus>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Transaction>, Self::AdapterError>;

    async fn count(
        &self,
        requester_id: Option<Id<User>>,
        status: Option<TransactionStatus>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<i64, Self::AdapterError>;
}
