use domain::{fraud_rule::result::FraudRuleResult, transaction::Transaction};
use entrait::entrait;
use lib::{anyhow::Result, async_trait, domain::Id};

#[entrait(
    FraudRuleResultRepositoryImpl,
    delegate_by=DelegateFraudRuleResultRepository
)]
#[async_trait]
pub trait FraudRuleResultRepository {
    async fn batch_create_fraud_rule_results(
        &self,
        source: (Id<Transaction>, Vec<FraudRuleResult>),
    ) -> Result<Vec<FraudRuleResult>>;

    async fn find_all_fraud_rule_results_by_transaction_id(
        &self,
        transaction_id: Id<Transaction>,
    ) -> Result<Vec<FraudRuleResult>>;
}
