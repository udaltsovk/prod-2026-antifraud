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
        validation::{ExternalInput, error::ValidationResultWithFields},
    },
};

use crate::usecase::user::error::UserUseCaseResult;

pub mod error;
pub mod implementation;

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
pub trait UserUseCase {
    async fn find_by_email(
        &self,
        user_email: &Email,
    ) -> UserUseCaseResult<Option<User>>;

    async fn get_by_email(&self, user_email: Email) -> UserUseCaseResult<User>;

    async fn create(
        &self,
        source: CreateUserSource,
        input: ValidationResultWithFields<CreateUser>,
    ) -> UserUseCaseResult<User>;

    async fn authorize(&self, input: CreateSession) -> UserUseCaseResult<User>;

    async fn find_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<Option<User>>;

    async fn get_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<User>;

    async fn list(
        &self,
        requester_role: UserRole,
        input: ValidationResultWithFields<Pagination>,
    ) -> UserUseCaseResult<(Vec<User>, u64)>;

    async fn update_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
        input: (
            ValidationResultWithFields<UserUpdate>,
            ExternalInput<bool>,
            ExternalInput<String>,
        ),
    ) -> UserUseCaseResult<User>;

    async fn deactivate_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<User>;
}
