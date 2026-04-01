use domain::user::{User, filter::UserFilterInput, role::UserRole};
use entrait::entrait;
use lib::{
    domain::validation::error::ValidationResultWithFields, tap::Pipe as _,
};
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    usecase::user::{UserUseCaseError, UserUseCaseResult},
};

#[entrait(pub ListUsersUsecase)]
#[instrument(skip(deps))]
async fn list_users<Deps>(
    deps: &Deps,
    requester_role: UserRole,
    input: ValidationResultWithFields<UserFilterInput>,
) -> UserUseCaseResult<(Vec<User>, u64)>
where
    Deps: UserRepository,
{
    if requester_role != UserRole::Admin {
        return UserUseCaseError::MissingPermissions.pipe(Err);
    }

    let user_filter = input.map_err(UserUseCaseError::Validation)?.normalize();

    let items = UserRepository::list_users(deps, user_filter).await?;

    let total = UserRepository::count_users(deps, user_filter).await?;

    Ok((items, total.try_into().unwrap_or(u64::MIN)))
}
