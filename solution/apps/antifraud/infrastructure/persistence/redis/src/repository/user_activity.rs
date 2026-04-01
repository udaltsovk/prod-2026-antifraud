use std::sync::LazyLock;

use application::repository::user_activity::UserActivityRepositoryImpl;
use domain::user::User;
use entrait::entrait;
use lib::{
    anyhow::Result,
    application::di::Has,
    async_trait,
    chrono::{DateTime, Utc},
    domain::Id,
    infrastructure::persistence::{HasPoolExt as _, redis::Namespace},
    instrument_all,
    tap::Pipe as _,
};
use mobc_redis::{RedisConnectionManager, mobc::Pool};
use redis::AsyncTypedCommands as _;

use crate::repository::{META_NAMESPACE, RedisRepositoryImpl};

static NAMESPACE: LazyLock<Namespace> =
    LazyLock::new(|| META_NAMESPACE.nest("user_activity"));

#[entrait(ref)]
#[async_trait]
#[instrument_all]
impl UserActivityRepositoryImpl for RedisRepositoryImpl {
    async fn record_activity<Deps>(
        deps: &Deps,
        user_id: Id<User>,
    ) -> Result<DateTime<Utc>>
    where
        Deps: Has<Pool<RedisConnectionManager>>,
    {
        let timestamp = Utc::now();

        let mut connection = deps.get_connection().await?;

        connection
            .set(
                NAMESPACE.key(&user_id.to_string()),
                timestamp.timestamp_millis(),
            )
            .await?;

        Ok(timestamp)
    }

    async fn find_activity_by_user<Deps>(
        deps: &Deps,
        user_id: Id<User>,
    ) -> Result<Option<DateTime<Utc>>>
    where
        Deps: Has<Pool<RedisConnectionManager>>,
    {
        let mut connection = deps.get_connection().await?;

        connection
            .get_int(NAMESPACE.key(&user_id.to_string()))
            .await?
            .and_then(|millis| millis.try_into().ok())
            .and_then(DateTime::from_timestamp_millis)
            .pipe(Ok)
    }
}
