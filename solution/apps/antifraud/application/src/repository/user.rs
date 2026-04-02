use domain::{
    email::Email,
    password::PasswordHash,
    user::{CreateUser, User, filter::UserFilter},
};
use entrait::entrait;
use lib::{anyhow::Result, async_trait, domain::Id};

#[entrait(
    UserRepositoryImpl,
    delegate_by=DelegateUserRepository
)]
#[async_trait]
pub trait UserRepository {
    async fn create_user(
        &self,
        source: (Id<User>, CreateUser, PasswordHash),
    ) -> Result<User>;

    async fn find_user_by_id(&self, id: Id<User>) -> Result<Option<User>>;

    async fn find_user_by_email(&self, email: &Email) -> Result<Option<User>>;

    async fn list_users(&self, filter: UserFilter) -> Result<Vec<User>>;

    async fn count_users(&self, filter: UserFilter) -> Result<i64>;

    async fn update_user(&self, source: User) -> Result<User>;
}
