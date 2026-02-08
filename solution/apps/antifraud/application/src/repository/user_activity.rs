use chrono::{DateTime, Utc};
use domain::user::User;
use lib::{anyhow::Result, async_trait, domain::Id};

#[async_trait]
pub trait UserActivityRepository {
    async fn record(&self, user_id: Id<User>) -> Result<DateTime<Utc>>;

    async fn find_by_user(
        &self,
        user_id: Id<User>,
    ) -> Result<Option<DateTime<Utc>>>;
}
