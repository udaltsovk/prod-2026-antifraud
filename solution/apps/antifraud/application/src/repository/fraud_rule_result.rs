use std::fmt::Debug;

use domain::{fraud_rule::result::FraudRuleResult, transaction::Transaction};
use lib::{async_trait, domain::Id};

#[async_trait]
pub trait FraudRuleResultRepository {
    type AdapterError: Debug + Send + Sync;

    async fn batch_create(
        &self,
        source: (Id<Transaction>, Vec<FraudRuleResult>),
    ) -> Result<Vec<FraudRuleResult>, Self::AdapterError>;

    async fn find_all_by_transaction_id(
        &self,
        transaction_id: Id<Transaction>,
    ) -> Result<Vec<FraudRuleResult>, Self::AdapterError>;
}
