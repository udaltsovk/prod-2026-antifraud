use std::fmt::Debug;

use domain::{
    email::Email,
    password_hash::PasswordHash,
    user::{CreateUser, User, is_active::UserStatus, role::UserRole},
};
use lib::{async_trait, domain::Id};

#[async_trait]
pub trait UserRepository {
    type AdapterError: Debug + Send + Sync;

    async fn create(
        &self,
        id: Id<User>,
        source: CreateUser,
        password_hash: PasswordHash,
    ) -> Result<User, Self::AdapterError>;

    async fn find_by_id(
        &self,
        id: Id<User>,
    ) -> Result<Option<User>, Self::AdapterError>;

    async fn find_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<User>, Self::AdapterError>;

    async fn list(
        &self,
        limit: i64,
        offset: i64,
        roles: Option<&[UserRole]>,
        status: Option<UserStatus>,
    ) -> Result<Vec<User>, Self::AdapterError>;

    async fn count(
        &self,
        roles: Option<&[UserRole]>,
        status: Option<UserStatus>,
    ) -> Result<i64, Self::AdapterError>;

    async fn update(&self, source: User) -> Result<User, Self::AdapterError>;
}
