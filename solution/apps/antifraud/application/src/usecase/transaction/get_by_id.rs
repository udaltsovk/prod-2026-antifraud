use domain::{
    transaction::{Transaction, decision::TransactionDecision},
    user::{User, role::UserRole},
};
use entrait::entrait;
use lib::domain::Id;
use tracing::instrument;

use crate::usecase::transaction::{
    TransactionUseCaseError, TransactionUseCaseResult,
    find_by_id::FindTransactionByIdUsecase,
};

#[entrait(pub GetTransactionByIdUsecase)]
#[instrument(skip(deps))]
async fn get_transaction_by_id<Deps>(
    deps: &Deps,
    requester: (Id<User>, UserRole),
    transaction_id: Id<Transaction>,
) -> TransactionUseCaseResult<TransactionDecision>
where
    Deps: FindTransactionByIdUsecase,
{
    FindTransactionByIdUsecase::find_transaction_by_id(
        deps,
        requester,
        transaction_id,
    )
    .await?
    .ok_or(TransactionUseCaseError::TransactionNotFoundById(
        transaction_id,
    ))
}
