use domain::user::{User, role::UserRole};
use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    usecase::user::{UserUseCaseError, UserUseCaseResult},
};

#[entrait(pub FindUserByIdUsecase)]
#[instrument(skip(deps))]
async fn find_user_by_id<Deps>(
    deps: &Deps,
    (requester_id, requester_role): (Id<User>, UserRole),
    user_id: Id<User>,
) -> UserUseCaseResult<Option<User>>
where
    Deps: UserRepository,
{
    if requester_role != UserRole::Admin && requester_id != user_id {
        return UserUseCaseError::MissingPermissions.pipe(Err);
    }

    UserRepository::find_user_by_id(deps, user_id)
        .await?
        .pipe(Ok)
}
