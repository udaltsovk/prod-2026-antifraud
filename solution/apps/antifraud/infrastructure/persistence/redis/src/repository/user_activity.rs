use std::sync::LazyLock;

use application::repository::user_activity::UserActivityRepository;
use domain::user::{User, UserActivity};
use lib::{
    anyhow::Result,
    async_trait,
    chrono::{DateTime, Utc},
    domain::Id,
    infrastructure::persistence::redis::Namespace,
    instrument_all,
    tap::Pipe as _,
};
use redis::AsyncTypedCommands as _;

use crate::repository::{META_NAMESPACE, RedisRepositoryImpl};

static NAMESPACE: LazyLock<Namespace> =
    LazyLock::new(|| META_NAMESPACE.nest("user_activity"));

#[async_trait]
#[instrument_all]
impl UserActivityRepository for RedisRepositoryImpl<UserActivity> {
    async fn record(&self, user_id: Id<User>) -> Result<DateTime<Utc>> {
        let timestamp = Utc::now();

        let mut connection = self.pool.get().await?;

        connection
            .set(
                NAMESPACE.key(&user_id.to_string()),
                timestamp.timestamp_millis(),
            )
            .await?;

        Ok(timestamp)
    }

    async fn find_by_user(
        &self,
        user_id: Id<User>,
    ) -> Result<Option<DateTime<Utc>>> {
        let mut connection = self.pool.get().await?;

        connection
            .get_int(NAMESPACE.key(&user_id.to_string()))
            .await?
            .and_then(|millis| millis.try_into().ok())
            .and_then(DateTime::from_timestamp_millis)
            .pipe(Ok)
    }
}
