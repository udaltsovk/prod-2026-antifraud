use domain::{
    statistics::users::UserRiskProfile,
    user::{User, role::UserRole},
};
use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::{
        statistics::StatisticsRepository, user::UserRepository,
        user_activity::UserActivityRepository,
    },
    usecase::statistics::{StatisticsUseCaseError, StatisticsUseCaseResult},
};

#[entrait(pub StatisticsUserRiskProfileUsecase)]
#[instrument(skip(deps))]
async fn statistics_user_risk_profile<Deps>(
    deps: &Deps,
    (requester_id, requester_role): (Id<User>, UserRole),
    user_id: Id<User>,
) -> StatisticsUseCaseResult<UserRiskProfile>
where
    Deps: UserRepository + StatisticsRepository + UserActivityRepository,
{
    if requester_role != UserRole::Admin && user_id != requester_id {
        return StatisticsUseCaseError::MissingPermissions.pipe(Err);
    }

    UserRepository::find_user_by_id(deps, user_id)
        .await?
        .ok_or(StatisticsUseCaseError::UserNotFoundById(user_id))?;

    let mut risk_profile =
        StatisticsRepository::statistics_user_risk_profile(deps, user_id)
            .await?;

    let user_activity =
        UserActivityRepository::find_activity_by_user(deps, user_id).await?;

    risk_profile.last_seen_at = user_activity;

    risk_profile.pipe(Ok)
}
