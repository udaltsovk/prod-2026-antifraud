use application::usecase::user::UserUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::{
    presentation::api::rest::{
        model::Parseable as _, response::ResponseExt as _,
    },
    tap::Pipe as _,
    uuid::Uuid,
};

use crate::{
    ModulesExt,
    errors::{ApiError, ApiResult},
    extractors::{
        Json, Path,
        session::{AdminSession, UserSession},
    },
    models::user::{JsonUser, JsonUserUpdate},
};

pub async fn get_user_by_id<M>(
    modules: State<M>,
    requester: UserSession,
    Path((_api_version, user_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .user_usecase()
        .get_by_id(requester.into(), user_id.into())
        .await
        .map(JsonUser::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn update_user_by_id<M>(
    modules: State<M>,
    requester: UserSession,
    Path((_api_version, user_id)): Path<((), Uuid)>,
    Json(update): Json<JsonUserUpdate>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let input = {
        let new_is_active = update.is_active.clone();
        let new_role = update.role.clone();
        (update.parse(), new_is_active.into(), new_role.into())
    };

    modules
        .user_usecase()
        .update_by_id(requester.into(), user_id.into(), input)
        .await
        .map(JsonUser::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn deactivate_user_by_id<M>(
    modules: State<M>,
    requester: AdminSession,
    Path((_api_version, user_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .user_usecase()
        .deactivate_by_id(requester.into(), user_id.into())
        .await
        .map_err(ApiError::from)
        .map(|_| StatusCode::NO_CONTENT)
}
