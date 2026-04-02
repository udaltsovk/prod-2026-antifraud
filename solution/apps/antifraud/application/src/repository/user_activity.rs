use chrono::{DateTime, Utc};
use domain::user::User;
use entrait::entrait;
use lib::{anyhow::Result, async_trait, domain::Id};

#[entrait(
    UserActivityRepositoryImpl,
    delegate_by=DelegateUserActivityRepository
)]
#[async_trait]
pub trait UserActivityRepository {
    async fn record_activity(&self, user_id: Id<User>)
    -> Result<DateTime<Utc>>;

    async fn find_activity_by_user(
        &self,
        user_id: Id<User>,
    ) -> Result<Option<DateTime<Utc>>>;
}
