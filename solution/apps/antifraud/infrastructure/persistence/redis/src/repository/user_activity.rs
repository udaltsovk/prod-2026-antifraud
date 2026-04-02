use std::sync::OnceLock;

use application::repository::user_activity::UserActivityRepositoryImpl;
use domain::user::User;
use entrait::entrait;
use lib::{
    anyhow::Result,
    application::di::Has,
    async_trait,
    chrono::{DateTime, Utc},
    domain::Id,
    infrastructure::persistence::{
        HasPoolExt as _,
        redis::{Namespace, RedisPool},
    },
    instrument_all,
    tap::Pipe as _,
};
use redis::AsyncTypedCommands as _;

use crate::repository::RedisRepositoryImpl;

#[entrait(HasUserActivityNamespace)]
fn user_activity_namespace<App>(app: &App) -> &Namespace
where
    App: Has<Namespace>,
{
    static NAMESPACE: OnceLock<Namespace> = OnceLock::new();
    NAMESPACE.get_or_init(|| app.get_dependency().nest("user_activity"))
}

#[entrait]
#[async_trait]
#[instrument_all]
impl UserActivityRepositoryImpl for RedisRepositoryImpl {
    async fn record_activity<App>(
        app: &App,
        user_id: Id<User>,
    ) -> Result<DateTime<Utc>>
    where
        App: Has<RedisPool> + HasUserActivityNamespace,
    {
        let timestamp = Utc::now();

        let mut connection = app.get_connection().await?;

        connection
            .set(
                app.user_activity_namespace().key(&user_id.to_string()),
                timestamp.timestamp_millis(),
            )
            .await?;

        Ok(timestamp)
    }

    async fn find_activity_by_user<App>(
        app: &App,
        user_id: Id<User>,
    ) -> Result<Option<DateTime<Utc>>>
    where
        App: Has<RedisPool> + HasUserActivityNamespace,
    {
        let mut connection = app.get_connection().await?;

        connection
            .get_int(app.user_activity_namespace().key(&user_id.to_string()))
            .await?
            .and_then(|millis| millis.try_into().ok())
            .and_then(DateTime::from_timestamp_millis)
            .pipe(Ok)
    }
}
