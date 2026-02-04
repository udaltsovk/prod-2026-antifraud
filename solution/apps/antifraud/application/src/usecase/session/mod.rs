use domain::{
    session::Session,
    user::{User, role::UserRole},
};
use lib::{async_trait, domain::Id, redact::Secret};

use crate::usecase::session::error::SessionUseCaseResult;

pub mod error;
pub mod implementation;

#[async_trait]
pub trait SessionUseCase {
    fn create(
        &self,
        user_id: Id<User>,
        user_role: UserRole,
    ) -> SessionUseCaseResult<Secret<String>>;

    fn get_from_token(
        &self,
        token: Secret<&str>,
    ) -> SessionUseCaseResult<Session>;
}
