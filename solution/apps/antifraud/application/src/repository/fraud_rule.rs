use std::fmt::Debug;

use domain::fraud_rule::{CreateFraudRule, FraudRule, name::FraudRuleName};
use lib::{async_trait, domain::Id};

#[async_trait]
pub trait FraudRuleRepository {
    type AdapterError: Debug + Send + Sync;

    async fn create(
        &self,
        id: Id<FraudRule>,
        source: CreateFraudRule,
    ) -> Result<FraudRule, Self::AdapterError>;

    async fn find_by_id(
        &self,
        id: Id<FraudRule>,
    ) -> Result<Option<FraudRule>, Self::AdapterError>;

    async fn find_by_name(
        &self,
        name: &FraudRuleName,
    ) -> Result<Option<FraudRule>, Self::AdapterError>;

    async fn list(&self) -> Result<Vec<FraudRule>, Self::AdapterError>;

    async fn update(
        &self,
        source: FraudRule,
    ) -> Result<FraudRule, Self::AdapterError>;
}
