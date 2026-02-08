use domain::{
    email::Email,
    password::PasswordHash,
    user::{CreateUser, User, filter::UserFilter},
};
use lib::{anyhow::Result, async_trait, domain::Id};

#[async_trait]
pub trait UserRepository {
    async fn create(
        &self,
        source: (Id<User>, CreateUser, PasswordHash),
    ) -> Result<User>;

    async fn find_by_id(&self, id: Id<User>) -> Result<Option<User>>;

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>>;

    async fn list(&self, filter: UserFilter) -> Result<Vec<User>>;

    async fn count(&self, filter: UserFilter) -> Result<i64>;

    async fn update(&self, source: User) -> Result<User>;
}
