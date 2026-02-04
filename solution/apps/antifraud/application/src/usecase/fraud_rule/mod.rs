use domain::{
    fraud_rule::{
        CreateFraudRule, FraudRule, FraudRuleUpdate,
        dsl_expression::FraudRuleDslExpression, name::FraudRuleName,
        status::FraudRuleStatus,
    },
    user::role::UserRole,
};
use lib::{
    async_trait,
    domain::{Id, validation::error::ValidationResultWithFields},
};

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
        creator_role: UserRole,
        source_result: ValidationResultWithFields<CreateFraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule>;

    async fn find_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<Option<FraudRule>>;

    async fn get_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule>;

    async fn list(
        &self,
        requester_role: UserRole,
        status: Option<FraudRuleStatus>,
    ) -> FraudRuleUseCaseResult<Vec<FraudRule>>;

    async fn update_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
        update_result: ValidationResultWithFields<FraudRuleUpdate>,
    ) -> FraudRuleUseCaseResult<FraudRule>;

    fn normalize_dsl_expression(
        &self,
        requester_role: UserRole,
        expression_result: ValidationResultWithFields<FraudRuleDslExpression>,
    ) -> FraudRuleUseCaseResult<DslServiceResult<FraudRuleDslExpression>>;

    async fn disable_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule>;
}
