use domain::user::{CreateUser, User, role::UserRole};
use entrait::entrait;
use lib::{
    domain::{Id, validation::error::ValidationResultWithFields},
    tap::Pipe as _,
};
use tracing::instrument;

use crate::{
    repository::user::UserRepository,
    service::secret_hasher::SecretHasherService,
    usecase::user::{
        CreateUserSource, FindUserByEmailUsecase, UserUseCaseError,
        UserUseCaseResult,
    },
};

#[entrait(pub CreateUserUsecase)]
#[instrument(skip(deps))]
async fn create_user<Deps>(
    deps: &Deps,
    source: CreateUserSource,
    input: ValidationResultWithFields<CreateUser>,
) -> UserUseCaseResult<User>
where
    Deps: FindUserByEmailUsecase + SecretHasherService + UserRepository,
{
    if source != CreateUserSource::User(UserRole::Admin)
        && source != CreateUserSource::Registration
    {
        return UserUseCaseError::MissingPermissions.pipe(Err);
    }

    let new_user = input.map_err(UserUseCaseError::Validation)?;

    if FindUserByEmailUsecase::find_user_by_email(deps, &new_user.email)
        .await?
        .is_some()
    {
        return UserUseCaseError::EmailAlreadyUsed(new_user.email).pipe(Err);
    }

    let password_hash = SecretHasherService::hash_secret(
        deps,
        &new_user.password.clone().into(),
    )?;

    UserRepository::create_user(deps, (Id::generate(), new_user, password_hash))
        .await?
        .pipe(Ok)
}
