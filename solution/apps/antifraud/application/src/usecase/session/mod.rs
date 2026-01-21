use domain::{
    session::Session,
    user::{User, role::UserRole},
};
use lib::{async_trait, domain::Id};

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::session::error::SessionUseCaseResult,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait SessionUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn create(
        &self,
        user_id: Id<User>,
        user_role: UserRole,
    ) -> SessionUseCaseResult<R, S, String>;

    fn get_from_token(
        &self,
        token: &str,
    ) -> SessionUseCaseResult<R, S, Session>;
}
