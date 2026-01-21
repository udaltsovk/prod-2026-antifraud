use domain::{
    pagination::Pagination,
    session::CreateSession,
    user::{CreateUser, User, role::UserRole},
};
use lib::{
    async_trait,
    domain::{Id, validation::error::ValidationResult},
};

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::user::error::UserUseCaseResult,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait UserUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn create(
        &self,
        creator_role: Option<UserRole>,
        source: ValidationResult<CreateUser>,
    ) -> UserUseCaseResult<R, S, User>;

    async fn authorize(
        &self,
        source: CreateSession,
    ) -> UserUseCaseResult<R, S, User>;

    async fn find_by_id(
        &self,
        requester_id: Id<User>,
        requester_role: UserRole,
        id: Id<User>,
    ) -> UserUseCaseResult<R, S, Option<User>>;

    async fn get_by_id(
        &self,
        requester_id: Id<User>,
        requester_role: UserRole,
        id: Id<User>,
    ) -> UserUseCaseResult<R, S, User>;

    async fn list(
        &self,
        requester_role: Option<UserRole>,
        pagination: ValidationResult<Pagination>,
        roles: Option<&[UserRole]>,
        is_active: Option<bool>,
    ) -> UserUseCaseResult<R, S, (Vec<User>, u64)>;
}
