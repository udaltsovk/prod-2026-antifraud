use domain::{email::Email, user::User};
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    repository::user::UserRepository, usecase::user::UserUseCaseResult,
};

#[entrait(pub FindUserByEmailUsecase)]
#[instrument(skip(deps))]
async fn find_user_by_email<Deps>(
    deps: &Deps,
    user_email: &Email,
) -> UserUseCaseResult<Option<User>>
where
    Deps: UserRepository,
{
    UserRepository::find_user_by_email(deps, user_email)
        .await?
        .pipe(Ok)
}
