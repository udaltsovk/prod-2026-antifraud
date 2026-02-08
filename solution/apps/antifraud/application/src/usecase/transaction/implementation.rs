use std::sync::Arc;

use domain::{
    fraud_rule::status::FraudRuleStatus,
    transaction::{
        CreateTransaction, Transaction, decision::TransactionDecision,
        filter::TransactionFilterInput,
    },
    user::{User, role::UserRole, status::UserStatus},
};
use futures::future;
use lib::{
    async_trait,
    domain::{
        Id,
        validation::{ExternalInput, error::ValidationResultWithFields},
    },
    instrument_all,
    tap::Pipe as _,
    uuid::Uuid,
};

use crate::usecase::{
    UseCase,
    transaction::{
        TransactionUseCase,
        error::{TransactionUseCaseError, TransactionUseCaseResult},
    },
};

#[async_trait]
#[instrument_all]
impl TransactionUseCase for UseCase<Transaction> {
    async fn create(
        &self,
        (creator_id, creator_role): (Id<User>, UserRole),
        input: (
            ValidationResultWithFields<CreateTransaction>,
            ExternalInput<Uuid>,
        ),
    ) -> TransactionUseCaseResult<TransactionDecision> {
        let creator = self
            .repositories
            .user()
            .find_by_id(creator_id)
            .await
            .map_err(TransactionUseCaseError::Infrastructure)?
            .map(Arc::new);

        let decision_tuple = self
            .get_transaction_decision_tuple(
                (creator_id, creator_role),
                creator.as_ref(),
                input,
            )
            .await?;

        let fraud_rules = self
            .repositories
            .fraud_rule()
            .list(FraudRuleStatus::Enabled.into())
            .await
            .map_err(TransactionUseCaseError::Infrastructure)?;

        let decisions = self
            .services
            .dsl()
            .decide(&fraud_rules, vec![(0, decision_tuple)])
            .into_iter()
            .map(|(_index, decision)| self.save_transaction_decision(decision))
            .pipe(future::join_all)
            .await;

        decisions
            .into_iter()
            .next()
            .expect("we've passed one transaction, so we should get it back")
    }

    async fn bulk_create(
        &self,
        (creator_id, creator_role): (Id<User>, UserRole),
        input: Vec<(
            ValidationResultWithFields<CreateTransaction>,
            ExternalInput<Uuid>,
        )>,
    ) -> TransactionUseCaseResult<
        Vec<(usize, TransactionUseCaseResult<TransactionDecision>)>,
    > {
        let creator = self
            .repositories
            .user()
            .find_by_id(creator_id)
            .await
            .map_err(TransactionUseCaseError::Infrastructure)?
            .map(Arc::new);

        let (decision_tuples, mut errors): (Vec<_>, Vec<_>) = input
            .into_iter()
            .enumerate()
            .map(async |(index, inp)| {
                let res = self
                    .get_transaction_decision_tuple(
                        (creator_id, creator_role),
                        creator.as_ref(),
                        inp,
                    )
                    .await;
                (index, res)
            })
            .pipe(future::join_all)
            .await
            .into_iter()
            .map(|(index, res)| match res {
                Ok(val) => (Some((index, val)), None),
                Err(err) => (None, Some((index, err))),
            })
            .unzip();

        let fraud_rules = self
            .repositories
            .fraud_rule()
            .list(FraudRuleStatus::Enabled.into())
            .await
            .map_err(TransactionUseCaseError::Infrastructure)?;

        let decision_tuples = decision_tuples.into_iter().flatten().collect();

        let (decisions, new_errors): (Vec<_>, Vec<_>) = self
            .services
            .dsl()
            .decide(&fraud_rules, decision_tuples)
            .into_iter()
            .map(async |(index, decision)| {
                let res = self.save_transaction_decision(decision).await;
                (index, res)
            })
            .pipe(future::join_all)
            .await
            .into_iter()
            .map(|(index, res)| match res {
                Ok(val) => (Some((index, val)), None),
                Err(err) => (None, Some((index, err))),
            })
            .unzip();

        errors.extend(new_errors);

        let mut results: Vec<_> = decisions
            .into_iter()
            .flatten()
            .map(|(index, decision)| (index, Ok(decision)))
            .collect();
        let errors: Vec<_> = errors
            .into_iter()
            .flatten()
            .map(|(index, error)| (index, Err(error)))
            .collect();

        results.extend(errors);

        Ok(results)
    }

    async fn find_by_id(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        transaction_id: Id<Transaction>,
    ) -> TransactionUseCaseResult<Option<TransactionDecision>> {
        let transaction = self
            .repositories
            .transaction()
            .find_by_id(transaction_id)
            .await
            .map_err(TransactionUseCaseError::Infrastructure)?;

        if let Some(transaction) = &transaction
            && requester_role != UserRole::Admin
            && transaction.user_id.as_ref() != &requester_id.value
        {
            return TransactionUseCaseError::MissingPermissions.pipe(Err);
        }

        if let Some(transaction) = transaction {
            let rule_results = self
                .repositories
                .fraud_rule_result()
                .find_all_by_transaction_id(transaction.id)
                .await
                .map_err(TransactionUseCaseError::Infrastructure)?;

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
    ) -> TransactionUseCaseResult<TransactionDecision> {
        self.find_by_id(requester, transaction_id).await?.ok_or(
            TransactionUseCaseError::TransactionNotFoundById(transaction_id),
        )
    }

    async fn list(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        (pagination_result, raw_user_id): (
            ValidationResultWithFields<TransactionFilterInput>,
            ExternalInput<Uuid>,
        ),
    ) -> TransactionUseCaseResult<(Vec<Transaction>, u64)> {
        if requester_role != UserRole::Admin && !raw_user_id.is_missing() {
            return TransactionUseCaseError::MissingPermissions.pipe(Err);
        }

        let transaction_filter = pagination_result
            .map_err(TransactionUseCaseError::Validation)?
            .normalize((requester_id, requester_role));

        let items = self
            .repositories
            .transaction()
            .list(transaction_filter)
            .await
            .map_err(TransactionUseCaseError::Infrastructure)?;

        let total = self
            .repositories
            .transaction()
            .count(transaction_filter)
            .await
            .map_err(TransactionUseCaseError::Infrastructure)?;

        Ok((items, total.try_into().unwrap_or(u64::MIN)))
    }
}

#[instrument_all]
impl UseCase<Transaction> {
    async fn check_user_by_id(
        &self,
        user_id: Id<User>,
        user: Option<&Arc<User>>,
    ) -> TransactionUseCaseResult<Arc<User>> {
        let user =
            user.ok_or(TransactionUseCaseError::UserNotFoundById(user_id))?;

        user.status
            .eq(&UserStatus::Active)
            .ok_or(TransactionUseCaseError::MissingPermissions)?;

        Ok(Arc::clone(user))
    }

    async fn get_transaction_decision_tuple(
        &self,
        (creator_id, creator_role): (Id<User>, UserRole),
        creator: Option<&Arc<User>>,
        (create_result, transaction_user_id): (
            ValidationResultWithFields<CreateTransaction>,
            ExternalInput<Uuid>,
        ),
    ) -> TransactionUseCaseResult<(CreateTransaction, Arc<User>)> {
        if transaction_user_id != ExternalInput::Ok(creator_id.into())
            && creator_role != UserRole::Admin
        {
            return TransactionUseCaseError::MissingPermissions.pipe(Err);
        }

        let input =
            create_result.map_err(TransactionUseCaseError::Validation)?;

        let user = self.check_user_by_id(creator_id, creator).await?;

        let user = if input.user_id.as_ref() == &creator_id.value {
            user
        } else {
            let user_id = (*input.user_id.as_ref()).into();
            let user_opt = self
                .repositories
                .user()
                .find_by_id(user_id)
                .await
                .map_err(TransactionUseCaseError::Infrastructure)?
                .map(Arc::new);
            self.check_user_by_id(user_id, user_opt.as_ref()).await?
        };

        Ok((input, user))
    }

    async fn save_transaction_decision(
        &self,
        TransactionDecision {
            transaction,
            rule_results,
        }: TransactionDecision,
    ) -> TransactionUseCaseResult<TransactionDecision> {
        let transaction = self
            .repositories
            .transaction()
            .save(transaction)
            .await
            .map_err(TransactionUseCaseError::Infrastructure)?;

        let rule_results = self
            .repositories
            .fraud_rule_result()
            .batch_create((transaction.id, rule_results))
            .await
            .map_err(TransactionUseCaseError::Infrastructure)?;

        TransactionDecision {
            transaction,
            rule_results,
        }
        .pipe(Ok)
    }
}
