use std::sync::Arc;

use domain::{
    fraud_rule::status::FraudRuleStatus,
    transaction::{CreateTransaction, decision::TransactionDecision},
    user::{User, role::UserRole},
};
use entrait::entrait;
use futures::future;
use lib::{
    anyhow::Context as _,
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

#[entrait(pub CreateTransactionUsecase)]
#[instrument(skip(deps))]
async fn create_transaction<Deps>(
    deps: &Deps,
    (creator_id, creator_role): (Id<User>, UserRole),
    input: (
        ValidationResultWithFields<CreateTransaction>,
        ExternalInput<Uuid>,
    ),
) -> TransactionUseCaseResult<TransactionDecision>
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

    let decision_tuple =
        GetTransactionDecisionTupleUsecase::get_transaction_decision_tuple(
            deps,
            (creator_id, creator_role),
            creator.as_ref(),
            input,
        )
        .await?;

    let fraud_rules = FraudRuleRepository::list_fraud_rules(
        deps,
        FraudRuleStatus::Enabled.into(),
    )
    .await?;

    DslService::decide(deps, &fraud_rules, vec![(0, decision_tuple)])
        .into_iter()
        .map(|(_index, decision)| {
            SaveTransactionDecisionUsecase::save_transaction_decision(
                deps, decision,
            )
        })
        .pipe(future::join_all)
        .await
        .into_iter()
        .next()
        .context("we've passed one transaction, so we should get it back")?
}
