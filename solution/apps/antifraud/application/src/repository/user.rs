use std::fmt::Debug;

use domain::{
    email::Email,
    user::{CreateUser, User, role::UserRole},
};
use lib::{async_trait, domain::Id};

#[async_trait]
pub trait UserRepository {
    type AdapterError: Debug + Send + Sync;

    async fn create(
        &self,
        id: Id<User>,
        source: CreateUser,
        password_hash: String,
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
        is_active: Option<bool>,
    ) -> Result<Vec<User>, Self::AdapterError>;

    async fn count(
        &self,
        roles: Option<&[UserRole]>,
        is_active: Option<bool>,
    ) -> Result<i64, Self::AdapterError>;
}
