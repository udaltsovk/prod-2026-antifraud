use std::sync::Arc;

use domain::{
    fraud_rule::status::FraudRuleStatus,
    transaction::{CreateTransaction, decision::TransactionDecision},
    user::{User, role::UserRole},
};
use entrait::entrait;
use futures::future;
use lib::{
    domain::{
        Id,
        validation::{ExternalInput, error::ValidationResultWithFields},
    },
    tap::Pipe as _,
    uuid::Uuid,
};
use tracing::instrument;

use crate::{
    repository::{fraud_rule::FraudRuleRepository, user::UserRepository},
    service::dsl::DslService,
    usecase::transaction::{
        TransactionUseCaseResult,
        get_decision_tuple::GetTransactionDecisionTupleUsecase,
        save_decision::SaveTransactionDecisionUsecase,
    },
};

#[entrait(pub BulkCreateTransactionsUsecase)]
#[instrument(skip(deps))]
async fn bulk_create_transactions<Deps>(
    deps: &Deps,
    (creator_id, creator_role): (Id<User>, UserRole),
    input: Vec<(
        ValidationResultWithFields<CreateTransaction>,
        ExternalInput<Uuid>,
    )>,
) -> TransactionUseCaseResult<
    Vec<(usize, TransactionUseCaseResult<TransactionDecision>)>,
>
where
    Deps: UserRepository
        + GetTransactionDecisionTupleUsecase
        + FraudRuleRepository
        + DslService
        + SaveTransactionDecisionUsecase,
{
    let creator = UserRepository::find_user_by_id(deps, creator_id)
        .await?
        .map(Arc::new);

    let (decision_tuples, mut errors): (Vec<_>, Vec<_>) = input
        .into_iter()
        .enumerate()
        .map(async |(index, inp)| {
            let res = GetTransactionDecisionTupleUsecase::get_transaction_decision_tuple(
                deps,
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

    let fraud_rules = FraudRuleRepository::list_fraud_rules(
        deps,
        FraudRuleStatus::Enabled.into(),
    )
    .await?;

    let decision_tuples = decision_tuples.into_iter().flatten().collect();

    let (decisions, new_errors): (Vec<_>, Vec<_>) =
        DslService::decide(deps, &fraud_rules, decision_tuples)
            .into_iter()
            .map(async |(index, decision)| {
                let res =
                    SaveTransactionDecisionUsecase::save_transaction_decision(
                        deps, decision,
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
