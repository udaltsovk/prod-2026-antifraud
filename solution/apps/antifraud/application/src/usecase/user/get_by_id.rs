use domain::user::{User, role::UserRole};
use entrait::entrait;
use lib::domain::Id;
use tracing::instrument;

use crate::usecase::user::{
    FindUserByIdUsecase, UserUseCaseError, UserUseCaseResult,
};

#[entrait(pub GetUserByIdUsecase)]
#[instrument(skip(deps))]
async fn get_user_by_id<Deps>(
    deps: &Deps,
    requester: (Id<User>, UserRole),
    user_id: Id<User>,
) -> UserUseCaseResult<User>
where
    Deps: FindUserByIdUsecase,
{
    FindUserByIdUsecase::find_user_by_id(deps, requester, user_id)
        .await?
        .ok_or(UserUseCaseError::NotFoundById(user_id))
}
