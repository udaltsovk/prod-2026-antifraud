use domain::{
    transaction::{Transaction, filter::TransactionFilterInput},
    user::{User, role::UserRole},
};
use entrait::entrait;
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
    repository::transaction::TransactionRepository,
    usecase::transaction::{TransactionUseCaseError, TransactionUseCaseResult},
};

#[entrait(pub ListTransactionsUsecase)]
#[instrument(skip(deps))]
async fn list_transactions<Deps>(
    deps: &Deps,
    (requester_id, requester_role): (Id<User>, UserRole),
    (pagination_result, raw_user_id): (
        ValidationResultWithFields<TransactionFilterInput>,
        ExternalInput<Uuid>,
    ),
) -> TransactionUseCaseResult<(Vec<Transaction>, u64)>
where
    Deps: TransactionRepository,
{
    if requester_role != UserRole::Admin && !raw_user_id.is_missing() {
        return TransactionUseCaseError::MissingPermissions.pipe(Err);
    }

    let transaction_filter = pagination_result
        .map_err(TransactionUseCaseError::Validation)?
        .normalize((requester_id, requester_role));

    let items =
        TransactionRepository::list_transactions(deps, transaction_filter)
            .await?;

    let total =
        TransactionRepository::count_transactions(deps, transaction_filter)
            .await?;

    Ok((items, total.try_into().unwrap_or(u64::MIN)))
}
