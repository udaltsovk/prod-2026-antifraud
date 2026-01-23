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
    extractors::{Json, Path, session::UserSession},
    models::user::{JsonUser, JsonUserUpdate},
};

pub async fn get_user_by_id<M>(
    modules: State<M>,
    UserSession {
        user_id: requester_id,
        user_role: requester_role,
    }: UserSession,
    Path((_api_version, user_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .user_usecase()
        .get_by_id(requester_id, requester_role, user_id.into())
        .await
        .map_err(ApiError::from)
        .map(JsonUser::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn update_user_by_id<M>(
    modules: State<M>,
    UserSession {
        user_id: requester_id,
        user_role: requester_role,
    }: UserSession,
    Path((_api_version, user_id)): Path<((), Uuid)>,
    Json(update): Json<JsonUserUpdate>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let (common_update, raw_admin_update) = update.into();
    let common_update_result = common_update.parse();

    modules
        .user_usecase()
        .update_by_id(
            requester_id,
            requester_role,
            user_id.into(),
            common_update_result,
            raw_admin_update,
        )
        .await
        .map_err(ApiError::from)
        .map(JsonUser::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn deactivate_user_by_id<M>(
    modules: State<M>,
    UserSession {
        user_id: requester_id,
        user_role: requester_role,
    }: UserSession,
    Path((_api_version, user_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .user_usecase()
        .deactivate_by_id(requester_id, requester_role, user_id.into())
        .await
        .map_err(ApiError::from)
        .map(|_| StatusCode::NOT_MODIFIED)
}
