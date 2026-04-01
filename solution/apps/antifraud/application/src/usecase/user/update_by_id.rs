use domain::user::{User, UserUpdate, role::UserRole};
use entrait::entrait;
use lib::{
    domain::{
        Id,
        validation::{ExternalInput, error::ValidationResultWithFields},
    },
    tap::Pipe as _,
};
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    usecase::user::{GetUserByIdUsecase, UserUseCaseError, UserUseCaseResult},
};

#[entrait(pub UpdateUserByIdUsecase)]
#[instrument(skip(deps))]
async fn update_user_by_id<Deps>(
    deps: &Deps,
    (requester_id, requester_role): (Id<User>, UserRole),
    user_id: Id<User>,
    (update_result, new_status, new_role): (
        ValidationResultWithFields<UserUpdate>,
        ExternalInput<bool>,
        ExternalInput<String>,
    ),
) -> UserUseCaseResult<User>
where
    Deps: GetUserByIdUsecase + UserRepository,
{
    if requester_role != UserRole::Admin
        && !(new_status.is_missing() && new_role.is_missing())
    {
        return UserUseCaseError::MissingPermissions.pipe(Err);
    }

    let user_update = update_result.map_err(UserUseCaseError::Validation)?;

    let user = GetUserByIdUsecase::get_user_by_id(
        deps,
        (requester_id, requester_role),
        user_id,
    )
    .await?;

    if user_update.eq(&user) {
        return Ok(user);
    }

    let updated_user = user_update.apply_to(user);

    UserRepository::update_user(deps, updated_user)
        .await?
        .pipe(Ok)
}
