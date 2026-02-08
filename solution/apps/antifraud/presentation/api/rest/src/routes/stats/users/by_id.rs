use application::usecase::statistics::StatisticsUseCase as _;
use axum::{extract::State, response::IntoResponse};
use lib::{tap::Pipe as _, uuid::Uuid};

use crate::{
    ModulesExt,
    dto::statistics::users::risk_profile::UserRiskProfileDto,
    errors::ApiResult,
    extractors::{Json, Path, session::UserSession},
};

pub async fn user_risk_profile_by_id<M>(
    modules: State<M>,
    requester: UserSession,
    Path(((), user_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .statistics_usecase()
        .user_risk_profile(requester.into(), user_id.into())
        .await?
        .pipe(UserRiskProfileDto::from)
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}
