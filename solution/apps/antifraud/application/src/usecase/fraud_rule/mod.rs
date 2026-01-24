use domain::{
    fraud_rule::{
        CreateFraudRule, FraudRule, FraudRuleUpdate,
        dsl_expression::FraudRuleDslExpression, name::FraudRuleName,
    },
    user::role::UserRole,
};
use lib::{
    async_trait,
    domain::{Id, validation::error::ValidationResult},
};

use crate::{
    repository::RepositoriesModuleExt,
    service::{ServicesModuleExt, dsl::DslServiceResult},
    usecase::fraud_rule::error::FraudRuleUseCaseResult,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait FraudRuleUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn find_by_name(
        &self,
        fraud_rule_name: &FraudRuleName,
    ) -> FraudRuleUseCaseResult<R, S, Option<FraudRule>>;

    async fn get_by_name(
        &self,
        fraud_rule_name: FraudRuleName,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule>;

    async fn create(
        &self,
        creator_role: UserRole,
        source_result: ValidationResult<CreateFraudRule>,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule>;

    async fn find_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<R, S, Option<FraudRule>>;

    async fn get_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule>;

    async fn list(
        &self,
        requester_role: UserRole,
    ) -> FraudRuleUseCaseResult<R, S, Vec<FraudRule>>;

    async fn update_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
        update_result: ValidationResult<FraudRuleUpdate>,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule>;

    fn normalize_dsl_expression(
        &self,
        requester_role: UserRole,
        expression_result: ValidationResult<FraudRuleDslExpression>,
    ) -> FraudRuleUseCaseResult<R, S, DslServiceResult<FraudRuleDslExpression>>;

    async fn disable_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule>;
}
