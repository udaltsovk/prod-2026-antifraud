use domain::fraud_rule::{
    CreateFraudRule, FraudRule, name::FraudRuleName, status::FraudRuleStatus,
};
use entrait::entrait;
use lib::{anyhow::Result, async_trait, domain::Id};

#[entrait(FraudRuleRepositoryImpl, delegate_by=ref)]
#[async_trait]
pub trait FraudRuleRepository {
    async fn create_fraud_rule(
        &self,
        source: (Id<FraudRule>, CreateFraudRule),
    ) -> Result<FraudRule>;

    async fn find_fraud_rule_by_id(
        &self,
        id: Id<FraudRule>,
    ) -> Result<Option<FraudRule>>;

    async fn find_fraud_rule_by_name(
        &self,
        name: &FraudRuleName,
    ) -> Result<Option<FraudRule>>;

    async fn list_fraud_rules(
        &self,
        status: Option<FraudRuleStatus>,
    ) -> Result<Vec<FraudRule>>;

    async fn update_fraud_rule(&self, source: FraudRule) -> Result<FraudRule>;
}
