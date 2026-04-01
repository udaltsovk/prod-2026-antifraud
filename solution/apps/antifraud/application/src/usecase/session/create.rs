use domain::{
    session::Session,
    user::{User, role::UserRole},
};
use entrait::entrait;
use lib::{domain::Id, redact::Secret, tap::Pipe as _};
use tracing::instrument;

use crate::{
    service::token::TokenService, usecase::session::SessionUseCaseResult,
};

#[entrait(pub CreateSessionUsecase)]
#[instrument(skip(deps))]
fn create_session<Deps>(
    deps: &Deps,
    user_id: Id<User>,
    user_role: UserRole,
) -> SessionUseCaseResult<Secret<String>>
where
    Deps: TokenService,
{
    let session = Session {
        user_id,
        user_role,
    };

    TokenService::generate_token(deps, session)?.pipe(Ok)
}
