use domain::fraud_rule::{
    CreateFraudRule, FraudRule, name::FraudRuleName, status::FraudRuleStatus,
};
use lib::{anyhow::Result, async_trait, domain::Id};

#[async_trait]
pub trait FraudRuleRepository {
    async fn create(
        &self,
        source: (Id<FraudRule>, CreateFraudRule),
    ) -> Result<FraudRule>;

    async fn find_by_id(&self, id: Id<FraudRule>) -> Result<Option<FraudRule>>;

    async fn find_by_name(
        &self,
        name: &FraudRuleName,
    ) -> Result<Option<FraudRule>>;

    async fn list(
        &self,
        status: Option<FraudRuleStatus>,
    ) -> Result<Vec<FraudRule>>;

    async fn update(&self, source: FraudRule) -> Result<FraudRule>;
}
