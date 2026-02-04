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
            .fraud_rule_repository()
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
        creator_role: UserRole,
        source: ValidationResultWithFields<CreateFraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule> {
        if creator_role != UserRole::Admin {
            return FraudRuleUseCaseError::MissingPermissions.pipe(Err);
        }

        let source = source.map_err(FraudRuleUseCaseError::Validation)?;

        if self.find_by_name(&source.name).await?.is_some() {
            return FraudRuleUseCaseError::NameAlreadyUsed(source.name)
                .pipe(Err);
        }

        self.repositories
            .fraud_rule_repository()
            .create((Id::generate(), source))
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)
    }

    async fn find_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<Option<FraudRule>> {
        if requester_role != UserRole::Admin {
            return FraudRuleUseCaseError::MissingPermissions.pipe(Err);
        }

        self.repositories
            .fraud_rule_repository()
            .find_by_id(fraud_rule_id)
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)?
            .pipe(Ok)
    }

    async fn get_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule> {
        self.find_by_id(requester_role, fraud_rule_id)
            .await?
            .ok_or(FraudRuleUseCaseError::NotFoundById(fraud_rule_id))
    }

    async fn list(
        &self,
        requester_role: UserRole,
        status: Option<FraudRuleStatus>,
    ) -> FraudRuleUseCaseResult<Vec<FraudRule>> {
        if requester_role != UserRole::Admin {
            return FraudRuleUseCaseError::MissingPermissions.pipe(Err);
        }

        self.repositories
            .fraud_rule_repository()
            .list(status)
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)
    }

    async fn update_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
        update_result: ValidationResultWithFields<FraudRuleUpdate>,
    ) -> FraudRuleUseCaseResult<FraudRule> {
        if requester_role != UserRole::Admin {
            return FraudRuleUseCaseError::MissingPermissions.pipe(Err);
        }

        let fraud_rule_update =
            update_result.map_err(FraudRuleUseCaseError::Validation)?;

        let fraud_rule = self.get_by_id(requester_role, fraud_rule_id).await?;

        if fraud_rule_update.eq(&fraud_rule) {
            return Ok(fraud_rule);
        }

        if let Some(rule) = self.find_by_name(&fraud_rule_update.name).await?
            && rule.id != fraud_rule.id
            && fraud_rule_update.name == rule.name
        {
            return FraudRuleUseCaseError::NameAlreadyUsed(
                fraud_rule_update.name,
            )
            .pipe(Err);
        }

        let updated_fraud_rule = fraud_rule_update.apply_to(fraud_rule);

        self.repositories
            .fraud_rule_repository()
            .update(updated_fraud_rule)
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)
    }

    fn normalize_dsl_expression(
        &self,
        requester_role: UserRole,
        expression_result: ValidationResultWithFields<FraudRuleDslExpression>,
    ) -> FraudRuleUseCaseResult<DslServiceResult<FraudRuleDslExpression>> {
        if requester_role != UserRole::Admin {
            return FraudRuleUseCaseError::MissingPermissions.pipe(Err);
        }

        let dsl_expression =
            expression_result.map_err(FraudRuleUseCaseError::Validation)?;

        self.services
            .dsl_service()
            .normalize(dsl_expression)
            .pipe(Ok)
    }

    async fn disable_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<FraudRule> {
        if requester_role != UserRole::Admin {
            return FraudRuleUseCaseError::MissingPermissions.pipe(Err);
        }

        let fraud_rule = self.get_by_id(requester_role, fraud_rule_id).await?;

        if fraud_rule.status == FraudRuleStatus::Disabled {
            return Ok(fraud_rule);
        }

        let updated_fraud_rule = fraud_rule.tap_mut(|fraud_rule| {
            fraud_rule.status = FraudRuleStatus::Disabled;
        });

        self.repositories
            .fraud_rule_repository()
            .update(updated_fraud_rule)
            .await
            .map_err(FraudRuleUseCaseError::Infrastructure)
    }
}
