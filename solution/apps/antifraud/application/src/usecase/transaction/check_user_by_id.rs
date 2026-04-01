use std::sync::Arc;

use domain::user::{User, status::UserStatus};
use entrait::entrait;
use lib::domain::Id;
use tracing::instrument;

use crate::usecase::transaction::{
    TransactionUseCaseError, TransactionUseCaseResult,
};

#[entrait(pub CheckTransactionUserByIdUsecase)]
#[instrument(skip(_deps))]
async fn check_transaction_user_by_id<Deps>(
    _deps: &Deps,
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
