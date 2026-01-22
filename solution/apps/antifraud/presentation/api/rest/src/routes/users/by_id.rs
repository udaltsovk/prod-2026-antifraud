use application::usecase::user::UserUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::{
    presentation::api::rest::response::ResponseExt as _,
    tap::{Conv as _, Pipe as _},
    uuid::Uuid,
};

use crate::{
    ModulesExt,
    errors::{ApiError, ApiResult},
    extractors::{Json, Path, session::UserSession},
    models::user::JsonUser,
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
        .map_err(ApiError::from)?
        .conv::<JsonUser>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
