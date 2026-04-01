use domain::{
    session::CreateSession,
    user::{User, status::UserStatus},
};
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    service::secret_hasher::SecretHasherService,
    usecase::user::{
        FindUserByEmailUsecase, UserUseCaseError, UserUseCaseResult,
    },
};

#[entrait(pub AuthorizeUserUsecase)]
#[instrument(skip(deps))]
async fn authorize_user<Deps>(
    deps: &Deps,
    input: CreateSession,
) -> UserUseCaseResult<User>
where
    Deps: FindUserByEmailUsecase + SecretHasherService,
{
    let user =
        FindUserByEmailUsecase::find_user_by_email(deps, &input.email).await?;

    SecretHasherService::verify_secret(
        deps,
        &input.password.clone().into(),
        user.as_ref().map(|u| &u.password_hash),
    )
    .map_err(|_| UserUseCaseError::InvalidPassword)?;

    let user = user.ok_or(UserUseCaseError::InvalidPassword)?;

    if user.status != UserStatus::Active {
        return UserUseCaseError::UserDeactivated.pipe(Err);
    }

    Ok(user)
}
