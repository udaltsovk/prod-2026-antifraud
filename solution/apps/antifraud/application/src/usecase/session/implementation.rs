use domain::{
    session::Session,
    user::{User, role::UserRole},
};
use lib::{async_trait, domain::Id, instrument_all, tap::Pipe as _};

use crate::{
    repository::RepositoriesModuleExt,
    service::{ServicesModuleExt, token::TokenService as _},
    usecase::{
        UseCase,
        session::{
            SessionUseCase,
            error::{SessionUseCaseError, SessionUseCaseResult},
        },
    },
};

#[async_trait]
#[instrument_all]
impl<R, S> SessionUseCase<R, S> for UseCase<R, S, Session>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn create(
        &self,
        user_id: Id<User>,
        user_role: UserRole,
    ) -> SessionUseCaseResult<R, S, String> {
        let session = Session {
            user_id,
            user_role,
        };

        self.services
            .token_service()
            .generate(session)
            .map_err(S::Error::from)
            .map_err(SessionUseCaseError::Service)
    }

    fn get_from_token(
        &self,
        token: &str,
    ) -> SessionUseCaseResult<R, S, Session> {
        self.services
            .token_service()
            .parse(token)
            .map_err(S::Error::from)
            .map_err(SessionUseCaseError::Service)?
            .pipe(Ok)
    }
}
