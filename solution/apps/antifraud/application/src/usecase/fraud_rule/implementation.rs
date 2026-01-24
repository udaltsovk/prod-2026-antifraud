use domain::{
    fraud_rule::{
        CreateFraudRule, FraudRule, FraudRuleUpdate, name::FraudRuleName,
        status::FraudRuleStatus,
    },
    user::role::UserRole,
};
use lib::{
    async_trait,
    domain::{Id, validation::error::ValidationResult},
    instrument_all,
    tap::{Pipe as _, Tap as _},
};

use crate::{
    repository::{RepositoriesModuleExt, fraud_rule::FraudRuleRepository as _},
    service::ServicesModuleExt,
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
impl<R, S> FraudRuleUseCase<R, S> for UseCase<R, S, FraudRule>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn find_by_name(
        &self,
        fraud_rule_name: &FraudRuleName,
    ) -> FraudRuleUseCaseResult<R, S, Option<FraudRule>> {
        self.repositories
            .fraud_rule_repository()
            .find_by_name(fraud_rule_name)
            .await
            .map_err(R::Error::from)
            .map_err(FraudRuleUseCaseError::Repository)
    }

    async fn get_by_name(
        &self,
        fraud_rule_name: FraudRuleName,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule> {
        self.find_by_name(&fraud_rule_name)
            .await?
            .ok_or(FraudRuleUseCaseError::NotFoundByName(fraud_rule_name))
    }

    async fn create(
        &self,
        creator_role: UserRole,
        source: ValidationResult<CreateFraudRule>,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule> {
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
            .create(Id::generate(), source)
            .await
            .map_err(R::Error::from)
            .map_err(FraudRuleUseCaseError::Repository)
    }

    async fn find_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<R, S, Option<FraudRule>> {
        if requester_role != UserRole::Admin {
            return FraudRuleUseCaseError::MissingPermissions.pipe(Err);
        }

        self.repositories
            .fraud_rule_repository()
            .find_by_id(fraud_rule_id)
            .await
            .map_err(R::Error::from)
            .map_err(FraudRuleUseCaseError::Repository)?
            .pipe(Ok)
    }

    async fn get_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule> {
        self.find_by_id(requester_role, fraud_rule_id)
            .await?
            .ok_or(FraudRuleUseCaseError::NotFoundById(fraud_rule_id))
    }

    async fn list(
        &self,
        requester_role: UserRole,
    ) -> FraudRuleUseCaseResult<R, S, Vec<FraudRule>> {
        if requester_role != UserRole::Admin {
            return FraudRuleUseCaseError::MissingPermissions.pipe(Err);
        }

        self.repositories
            .fraud_rule_repository()
            .list()
            .await
            .map_err(R::Error::from)
            .map_err(FraudRuleUseCaseError::Repository)
    }

    async fn update_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
        update_result: ValidationResult<FraudRuleUpdate>,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule> {
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
            .map_err(R::Error::from)
            .map_err(FraudRuleUseCaseError::Repository)
    }

    async fn disable_by_id(
        &self,
        requester_role: UserRole,
        fraud_rule_id: Id<FraudRule>,
    ) -> FraudRuleUseCaseResult<R, S, FraudRule> {
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
            .map_err(R::Error::from)
            .map_err(FraudRuleUseCaseError::Repository)
    }
}
