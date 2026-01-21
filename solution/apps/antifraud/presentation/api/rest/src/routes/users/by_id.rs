use application::usecase::user::UserUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::{
    presentation::api::rest::response::ResponseExt as _,
    tap::{Conv as _, Pipe as _},
    uuid::Uuid,
};

use crate::{
    ModulesExt,
    errors::ApiError,
    extractors::{Json, Path, session::UserSession},
    models::user::JsonUser,
};

pub async fn get_user_by_id<M: ModulesExt>(
    modules: State<M>,
    user_session: UserSession,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    modules
        .user_usecase()
        .get_by_id(user_session.user_id, user_session.user_role, id.into())
        .await
        .map_err(ApiError::from)?
        .conv::<JsonUser>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
