use domain::{
    email::Email,
    pagination::Pagination,
    session::CreateSession,
    user::{CreateUser, User, UserUpdate, role::UserRole},
};
use lib::{
    async_trait,
    domain::{
        Id,
        validation::{ExternalInput, error::ValidationResult},
    },
};

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::user::error::UserUseCaseResult,
};

pub mod error;
pub mod implementation;

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum GetUserByEmailSource {
    Auth,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum CreateUserSource {
    Registration,
    User(UserRole),
}

impl From<UserRole> for CreateUserSource {
    fn from(creator_role: UserRole) -> Self {
        Self::User(creator_role)
    }
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
        source: GetUserByEmailSource,
    ) -> UserUseCaseResult<R, S, User>;

    async fn create(
        &self,
        source: CreateUserSource,
        input: ValidationResult<CreateUser>,
    ) -> UserUseCaseResult<R, S, User>;

    async fn authorize(
        &self,
        input: CreateSession,
    ) -> UserUseCaseResult<R, S, User>;

    async fn find_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, Option<User>>;

    async fn get_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, User>;

    async fn list(
        &self,
        requester_role: UserRole,
        input: ValidationResult<Pagination>,
    ) -> UserUseCaseResult<R, S, (Vec<User>, u64)>;

    async fn update_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
        input: (
            ValidationResult<UserUpdate>,
            ExternalInput<bool>,
            ExternalInput<String>,
        ),
    ) -> UserUseCaseResult<R, S, User>;

    async fn deactivate_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, User>;
}
