use domain::fraud_rule::{
    CreateFraudRule, FraudRule, FraudRuleUpdate,
    dsl_expression::FraudRuleDslExpression, name::FraudRuleName,
    status::FraudRuleStatus,
};
use lib::{
    async_trait,
    domain::Id,
    instrument_all,
    tap::{Pipe as _, Tap as _},
};

use crate::{
    service::dsl::DslServiceResult,
    usecase::{
        UseCase,
        fraud_rule::{
            FraudRuleUseCase,
            error::{FraudRuleUseCaseError, FraudRuleUseCaseResult},
        },
    },
};

#[async_trait]
#[instrument_all]
impl FraudRuleUseCase for UseCase<FraudRule> {
    async fn find_by_name(
        &self,
        fraud_rule_name: &FraudRuleName,
    ) -> FraudRuleUseCaseResult<Option<FraudRule>> {
        self.repositories
            .fraud_rule()
            .find_by_name(fraud_rule_name)
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)
    }

    async fn get_by_name(
        &self,
        fraud_rule_name: FraudRuleName,
    ) -> FraudRuleUseCaseResult<FraudRule> {
        self.find_by_name(&fraud_rule_name)
            .await?
            .ok_or(FraudRuleUseCaseError::NotFoundByName(fraud_rule_name))
    }

    async fn create(
        &self,
        source: CreateFraudRule,
    ) -> FraudRuleUseCaseResult<FraudRule> {
        if self.find_by_name(&source.name).await?.is_some() {
            return FraudRuleUseCaseError::NameAlreadyUsed(source.name)
                .pipe(Err);
        }

        self.repositories
            .fraud_rule()
            .create((Id::generate(), source))
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)
    }

    async fn find_by_id(
        &self,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<Option<FraudRule>> {
        self.repositories
            .fraud_rule()
            .find_by_id(fraud_rule_id)
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)?
            .pipe(Ok)
    }

    async fn get_by_id(
        &self,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule> {
        self.find_by_id(fraud_rule_id)
            .await?
            .ok_or(FraudRuleUseCaseError::NotFoundById(fraud_rule_id))
    }

    async fn list(
        &self,
        status: Option<FraudRuleStatus>,
    ) -> FraudRuleUseCaseResult<Vec<FraudRule>> {
        self.repositories
            .fraud_rule()
            .list(status)
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)
    }

    async fn update_by_id(
        &self,
        fraud_rule_id: Id<FraudRule>,
        update: FraudRuleUpdate,
    ) -> FraudRuleUseCaseResult<FraudRule> {
        let fraud_rule = self.get_by_id(fraud_rule_id).await?;

        if update.eq(&fraud_rule) {
            return Ok(fraud_rule);
        }

        if let Some(rule) = self.find_by_name(&update.name).await?
            && rule.id != fraud_rule.id
            && update.name == rule.name
        {
            return FraudRuleUseCaseError::NameAlreadyUsed(update.name)
                .pipe(Err);
        }

        let updated_fraud_rule = update.apply_to(fraud_rule);

        self.repositories
            .fraud_rule()
            .update(updated_fraud_rule)
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)
    }

    fn normalize_dsl_expression(
        &self,
        expression: FraudRuleDslExpression,
    ) -> FraudRuleUseCaseResult<DslServiceResult<FraudRuleDslExpression>> {
        self.services.dsl().normalize(expression).pipe(Ok)
    }

    async fn disable_by_id(
        &self,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule> {
        let fraud_rule = self.get_by_id(fraud_rule_id).await?;

        if fraud_rule.status == FraudRuleStatus::Disabled {
            return Ok(fraud_rule);
        }

        let updated_fraud_rule = fraud_rule.tap_mut(|fraud_rule| {
            fraud_rule.status = FraudRuleStatus::Disabled;
        });

        self.repositories
            .fraud_rule()
            .update(updated_fraud_rule)
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)
    }
}
