use domain::{email::Email, user::User};
use entrait::entrait;
use tracing::instrument;

use crate::usecase::user::{
    FindUserByEmailUsecase, UserUseCaseError, UserUseCaseResult,
};

#[entrait(pub GetUserByEmailUsecase)]
#[instrument(skip(deps))]
async fn get_user_by_email<Deps>(
    deps: &Deps,
    user_email: Email,
) -> UserUseCaseResult<User>
where
    Deps: FindUserByEmailUsecase,
{
    FindUserByEmailUsecase::find_user_by_email(deps, &user_email)
        .await?
        .ok_or(UserUseCaseError::NotFoundByEmail(user_email))
}
