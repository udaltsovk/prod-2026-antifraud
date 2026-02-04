use domain::{fraud_rule::result::FraudRuleResult, transaction::Transaction};
use lib::{anyhow::Result, async_trait, domain::Id};

#[async_trait]
pub trait FraudRuleResultRepository {
    async fn batch_create(
        &self,
        source: (Id<Transaction>, Vec<FraudRuleResult>),
    ) -> Result<Vec<FraudRuleResult>>;

    async fn find_all_by_transaction_id(
        &self,
        transaction_id: Id<Transaction>,
    ) -> Result<Vec<FraudRuleResult>>;
}
