use domain::session::Session;
use entrait::entrait;
use lib::{redact::Secret, tap::Pipe as _};
use tracing::instrument;

use crate::{
    service::token::TokenService, usecase::session::SessionUseCaseResult,
};

#[entrait(pub GetSessionFromTokenUsecase)]
#[instrument(skip(deps))]
fn get_session_from_token<Deps>(
    deps: &Deps,
    token: Secret<&str>,
) -> SessionUseCaseResult<Session>
where
    Deps: TokenService,
{
    TokenService::parse_token(deps, token)?.pipe(Ok)
}
