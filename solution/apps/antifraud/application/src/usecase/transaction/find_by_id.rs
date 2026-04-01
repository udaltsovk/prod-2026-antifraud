use domain::{
    transaction::{Transaction, decision::TransactionDecision},
    user::{User, role::UserRole},
};
use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::{
        fraud_rule_result::FraudRuleResultRepository,
        transaction::TransactionRepository,
    },
    usecase::transaction::{TransactionUseCaseError, TransactionUseCaseResult},
};

#[entrait(pub FindTransactionByIdUsecase)]
#[instrument(skip(deps))]
async fn find_transaction_by_id<Deps>(
    deps: &Deps,
    (requester_id, requester_role): (Id<User>, UserRole),
    transaction_id: Id<Transaction>,
) -> TransactionUseCaseResult<Option<TransactionDecision>>
where
    Deps: TransactionRepository + FraudRuleResultRepository,
{
    let transaction =
        TransactionRepository::find_transaction_by_id(deps, transaction_id)
            .await?;

    if let Some(transaction) = &transaction
        && requester_role != UserRole::Admin
        && transaction.user_id.as_ref() != &requester_id.value
    {
        return TransactionUseCaseError::MissingPermissions.pipe(Err);
    }

    let Some(transaction) = transaction else {
        return Ok(None);
    };

    let rule_results = FraudRuleResultRepository::find_all_fraud_rule_results_by_transaction_id(
        deps,
        transaction.id
    ).await?;

    TransactionDecision {
        transaction,
        rule_results,
    }
    .pipe(Some)
    .pipe(Ok)
}
