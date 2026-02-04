use domain::{
    session::Session,
    user::{User, role::UserRole},
};
use lib::{
    async_trait, domain::Id, instrument_all, redact::Secret, tap::Pipe as _,
};

use crate::usecase::{
    UseCase,
    session::{
        SessionUseCase,
        error::{SessionUseCaseError, SessionUseCaseResult},
    },
};

#[async_trait]
#[instrument_all]
impl SessionUseCase for UseCase<Session> {
    fn create(
        &self,
        user_id: Id<User>,
        user_role: UserRole,
    ) -> SessionUseCaseResult<Secret<String>> {
        let session = Session {
            user_id,
            user_role,
        };

        self.services
            .token_service()
            .generate(session)
            .map_err(SessionUseCaseError::Infrastructure)
    }

    fn get_from_token(
        &self,
        token: Secret<&str>,
    ) -> SessionUseCaseResult<Session> {
        self.services
            .token_service()
            .parse(token)
            .map_err(SessionUseCaseError::Infrastructure)?
            .pipe(Ok)
    }
}
