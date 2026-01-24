use domain::{
    fraud_rule::status::FraudRuleStatus,
    transaction::{
        CreateTransaction, Transaction, decision::TransactionDecision,
        pagination::TransactionPagination,
    },
    user::{User, role::UserRole, status::UserStatus},
};
use lib::{
    async_trait,
    domain::{
        Id,
        validation::{ExternalInput, error::ValidationResult},
    },
    instrument_all,
    tap::Pipe as _,
    uuid::Uuid,
};

use crate::{
    repository::{
        RepositoriesModuleExt, fraud_rule::FraudRuleRepository as _,
        fraud_rule_result::FraudRuleResultRepository as _,
        transaction::TransactionRepository as _, user::UserRepository as _,
    },
    service::{ServicesModuleExt, dsl::DslService as _},
    usecase::{
        UseCase,
        transaction::{
            TransactionUseCase,
            error::{TransactionUseCaseError, TransactionUseCaseResult},
        },
    },
};

#[async_trait]
#[instrument_all]
impl<R, S> TransactionUseCase<R, S> for UseCase<R, S, Transaction>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn create(
        &self,
        (creator_id, creator_role): (Id<User>, UserRole),
        (create_result, transaction_user_id): (
            ValidationResult<CreateTransaction>,
            ExternalInput<Uuid>,
        ),
    ) -> TransactionUseCaseResult<R, S, TransactionDecision> {
        if transaction_user_id != ExternalInput::Ok(creator_id.into())
            && creator_role != UserRole::Admin
        {
            return TransactionUseCaseError::MissingPermissions.pipe(Err);
        }

        let input =
            create_result.map_err(TransactionUseCaseError::Validation)?;

        let user = self.check_user_by_id(creator_id).await?;

        let user = if input.user_id.as_ref() == &creator_id.value {
            user
        } else {
            self.check_user_by_id((*input.user_id.as_ref()).into())
                .await?
        };

        let fraud_rules = self
            .repositories
            .fraud_rule_repository()
            .list(FraudRuleStatus::Enabled.into())
            .await
            .map_err(R::Error::from)
            .map_err(TransactionUseCaseError::Repository)?;

        let TransactionDecision {
            transaction,
            rule_results,
        } = self
            .services
            .dsl_service()
            .decide(&fraud_rules, input, &user);

        let transaction = self
            .repositories
            .transaction_repository()
            .save(transaction)
            .await
            .map_err(R::Error::from)
            .map_err(TransactionUseCaseError::Repository)?;

        let rule_results = self
            .repositories
            .fraud_rule_result_repository()
            .batch_create((transaction.id, rule_results))
            .await
            .map_err(R::Error::from)
            .map_err(TransactionUseCaseError::Repository)?;

        TransactionDecision {
            transaction,
            rule_results,
        }
        .pipe(Ok)
    }

    async fn bulk_create(
        &self,
        _creator: (Id<User>, UserRole),
        _create_result: ValidationResult<CreateTransaction>,
    ) -> Vec<(i64, TransactionUseCaseResult<R, S, TransactionDecision>)> {
        todo!()
    }

    async fn find_by_id(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        transaction_id: Id<Transaction>,
    ) -> TransactionUseCaseResult<R, S, Option<TransactionDecision>> {
        let transaction = self
            .repositories
            .transaction_repository()
            .find_by_id(transaction_id)
            .await
            .map_err(R::Error::from)
            .map_err(TransactionUseCaseError::Repository)?;

        if let Some(transaction) = &transaction
            && requester_role != UserRole::Admin
            && transaction.user_id.as_ref() != &requester_id.value
        {
            return TransactionUseCaseError::MissingPermissions.pipe(Err);
        }

        if let Some(transaction) = transaction {
            let rule_results = self
                .repositories
                .fraud_rule_result_repository()
                .find_all_by_transaction_id(transaction.id)
                .await
                .map_err(R::Error::from)
                .map_err(TransactionUseCaseError::Repository)?;

            TransactionDecision {
                transaction,
                rule_results,
            }
            .pipe(Some)
        } else {
            None
        }
        .pipe(Ok)
    }

    async fn get_by_id(
        &self,
        requester: (Id<User>, UserRole),
        transaction_id: Id<Transaction>,
    ) -> TransactionUseCaseResult<R, S, TransactionDecision> {
        self.find_by_id(requester, transaction_id).await?.ok_or(
            TransactionUseCaseError::TransactionNotFoundById(transaction_id),
        )
    }

    async fn list(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        (pagination_result, raw_user_id): (
            ValidationResult<TransactionPagination>,
            ExternalInput<Uuid>,
        ),
    ) -> TransactionUseCaseResult<R, S, (Vec<Transaction>, u64)> {
        if requester_role != UserRole::Admin && !raw_user_id.is_missing() {
            return TransactionUseCaseError::MissingPermissions.pipe(Err);
        }

        let (user_id, status, from, to, limit, offset) = pagination_result
            .map_err(TransactionUseCaseError::Validation)?
            .into_parts((requester_id, requester_role));

        let items = self
            .repositories
            .transaction_repository()
            .list(user_id, status, from, to, limit, offset)
            .await
            .map_err(R::Error::from)
            .map_err(TransactionUseCaseError::Repository)?;

        let total = self
            .repositories
            .transaction_repository()
            .count(user_id, status, from, to)
            .await
            .map_err(R::Error::from)
            .map_err(TransactionUseCaseError::Repository)?;

        Ok((items, total.try_into().unwrap_or(u64::MIN)))
    }
}

#[instrument_all]
impl<R, S> UseCase<R, S, Transaction>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn check_user_by_id(
        &self,
        user_id: Id<User>,
    ) -> TransactionUseCaseResult<R, S, User> {
        let user = self
            .repositories
            .user_repository()
            .find_by_id(user_id)
            .await
            .map_err(R::Error::from)
            .map_err(TransactionUseCaseError::Repository)?
            .ok_or(TransactionUseCaseError::UserNotFoundById(user_id))?;

        user.status
            .eq(&UserStatus::Active)
            .ok_or(TransactionUseCaseError::MissingPermissions)?;

        Ok(user)
    }
}
