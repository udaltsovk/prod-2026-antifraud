use domain::{
    email::Email,
    pagination::Pagination,
    session::CreateSession,
    user::{
        CreateUser, RawUserAdminUpdate, User, UserCommonUpdate,
        is_active::UserStatus, role::UserRole,
    },
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GetByEmailSource {
    Auth,
}

#[async_trait]
pub trait UserUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn find_by_email(
        &self,
        user_email: &Email,
    ) -> UserUseCaseResult<R, S, Option<User>>;

    async fn get_by_email(
        &self,
        user_email: Email,
        source: GetByEmailSource,
    ) -> UserUseCaseResult<R, S, User>;

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
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, Option<User>>;

    async fn get_by_id(
        &self,
        requester_id: Id<User>,
        requester_role: UserRole,
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, User>;

    async fn list(
        &self,
        requester_role: Option<UserRole>,
        pagination_result: ValidationResult<Pagination>,
        roles: Option<&[UserRole]>,
        status: Option<UserStatus>,
    ) -> UserUseCaseResult<R, S, (Vec<User>, u64)>;

    async fn update_by_id(
        &self,
        requester_id: Id<User>,
        requester_role: UserRole,
        user_id: Id<User>,
        common_update_result: ValidationResult<UserCommonUpdate>,
        raw_admin_update: RawUserAdminUpdate,
    ) -> UserUseCaseResult<R, S, User>;

    async fn deactivate_by_id(
        &self,
        requester_id: Id<User>,
        requester_role: UserRole,
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, User>;
}
