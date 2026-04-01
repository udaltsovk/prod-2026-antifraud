use chrono::{DateTime, Utc};
use domain::user::User;
use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};

use crate::{
    repository::user_activity::UserActivityRepository,
    usecase::user::UserUseCaseResult,
};

#[entrait(pub RecordUserActivityUsecase)]
async fn record_user_activity<Deps>(
    deps: &Deps,
    user_id: Id<User>,
) -> UserUseCaseResult<DateTime<Utc>>
where
    Deps: UserActivityRepository,
{
    UserActivityRepository::record_activity(deps, user_id)
        .await?
        .pipe(Ok)
}
