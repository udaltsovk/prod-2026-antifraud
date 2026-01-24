use std::fmt::Debug;

use domain::fraud_rule::{
    CreateFraudRule, FraudRule, name::FraudRuleName, status::FraudRuleStatus,
};
use lib::{async_trait, domain::Id};

#[async_trait]
pub trait FraudRuleRepository {
    type AdapterError: Debug + Send + Sync;

    async fn create(
        &self,
        source: (Id<FraudRule>, CreateFraudRule),
    ) -> Result<FraudRule, Self::AdapterError>;

    async fn find_by_id(
        &self,
        id: Id<FraudRule>,
    ) -> Result<Option<FraudRule>, Self::AdapterError>;

    async fn find_by_name(
        &self,
        name: &FraudRuleName,
    ) -> Result<Option<FraudRule>, Self::AdapterError>;

    async fn list(
        &self,
        status: Option<FraudRuleStatus>,
    ) -> Result<Vec<FraudRule>, Self::AdapterError>;

    async fn update(
        &self,
        source: FraudRule,
    ) -> Result<FraudRule, Self::AdapterError>;
}
