use domain::user::{User, role::UserRole, status::UserStatus};
use entrait::entrait;
use lib::{
    domain::Id,
    tap::{Pipe as _, Tap as _},
};
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    usecase::user::{GetUserByIdUsecase, UserUseCaseError, UserUseCaseResult},
};

#[entrait(pub DeactivateUserByIdUsecase)]
#[instrument(skip(deps))]
async fn deactivate_user_by_id<Deps>(
    deps: &Deps,
    (requester_id, requester_role): (Id<User>, UserRole),
    user_id: Id<User>,
) -> UserUseCaseResult<User>
where
    Deps: GetUserByIdUsecase + UserRepository,
{
    if requester_role != UserRole::Admin {
        return UserUseCaseError::MissingPermissions.pipe(Err);
    }

    let user = GetUserByIdUsecase::get_user_by_id(
        deps,
        (requester_id, requester_role),
        user_id,
    )
    .await?;

    if user.status == UserStatus::Deactivated {
        return Ok(user);
    }

    let updated_user =
        user.tap_mut(|user| user.status = UserStatus::Deactivated);

    UserRepository::update_user(deps, updated_user)
        .await?
        .pipe(Ok)
}
