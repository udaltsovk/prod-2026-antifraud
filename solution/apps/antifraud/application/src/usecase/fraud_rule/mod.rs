use domain::fraud_rule::{
    CreateFraudRule, FraudRule, FraudRuleUpdate,
    dsl_expression::FraudRuleDslExpression, name::FraudRuleName,
    status::FraudRuleStatus,
};
use lib::{async_trait, domain::Id};

use crate::{
    service::dsl::DslServiceResult,
    usecase::fraud_rule::error::FraudRuleUseCaseResult,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait FraudRuleUseCase {
    async fn find_by_name(
        &self,
        fraud_rule_name: &FraudRuleName,
    ) -> FraudRuleUseCaseResult<Option<FraudRule>>;

    async fn get_by_name(
        &self,
        fraud_rule_name: FraudRuleName,
    ) -> FraudRuleUseCaseResult<FraudRule>;

    async fn create(
        &self,
        source: CreateFraudRule,
    ) -> FraudRuleUseCaseResult<FraudRule>;

    async fn find_by_id(
        &self,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<Option<FraudRule>>;

    async fn get_by_id(
        &self,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule>;

    async fn list(
        &self,
        status: Option<FraudRuleStatus>,
    ) -> FraudRuleUseCaseResult<Vec<FraudRule>>;

    async fn update_by_id(
        &self,
        fraud_rule_id: Id<FraudRule>,
        update: FraudRuleUpdate,
    ) -> FraudRuleUseCaseResult<FraudRule>;

    fn normalize_dsl_expression(
        &self,
        expression: FraudRuleDslExpression,
    ) -> FraudRuleUseCaseResult<DslServiceResult<FraudRuleDslExpression>>;

    async fn disable_by_id(
        &self,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule>;
}
