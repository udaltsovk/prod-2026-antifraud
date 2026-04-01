use std::sync::Arc;

use domain::{
    transaction::CreateTransaction,
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
    repository::user::UserRepository,
    usecase::transaction::{
        TransactionUseCaseError, TransactionUseCaseResult,
        check_user_by_id::CheckTransactionUserByIdUsecase,
    },
};

#[entrait(pub GetTransactionDecisionTupleUsecase)]
#[instrument(skip(deps))]
async fn get_transaction_decision_tuple<Deps>(
    deps: &Deps,
    (creator_id, creator_role): (Id<User>, UserRole),
    creator: Option<&Arc<User>>,
    (create_result, transaction_user_id): (
        ValidationResultWithFields<CreateTransaction>,
        ExternalInput<Uuid>,
    ),
) -> TransactionUseCaseResult<(CreateTransaction, Arc<User>)>
where
    Deps: CheckTransactionUserByIdUsecase + UserRepository,
{
    if transaction_user_id != ExternalInput::Ok(creator_id.into())
        && creator_role != UserRole::Admin
    {
        return TransactionUseCaseError::MissingPermissions.pipe(Err);
    }

    let input = create_result.map_err(TransactionUseCaseError::Validation)?;

    let user = CheckTransactionUserByIdUsecase::check_transaction_user_by_id(
        deps, creator_id, creator,
    )
    .await?;

    let user = if input.user_id.as_ref() == &creator_id.value {
        user
    } else {
        let user_id = (*input.user_id.as_ref()).into();
        let user_opt = UserRepository::find_user_by_id(deps, user_id)
            .await?
            .map(Arc::new);

        CheckTransactionUserByIdUsecase::check_transaction_user_by_id(
            deps,
            user_id,
            user_opt.as_ref(),
        )
        .await?
    };

    Ok((input, user))
}
