use application::usecase::user::{
    DeactivateUserByIdUsecase, GetUserByIdUsecase, UpdateUserByIdUsecase,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::{
    presentation::api::rest::{
        response::ResponseExt as _, validation::parseable::Parseable as _,
    },
    tap::Pipe as _,
    uuid::Uuid,
};

use crate::{
    dto::user::{UserDto, UserUpdateDto},
    errors::ApiResult,
    extractors::{
        Json, Path,
        session::{AdminSession, UserSession},
    },
};

pub async fn get_user_by_id<App>(
    app: State<App>,
    requester: UserSession,
    Path(((), user_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    App: GetUserByIdUsecase,
{
    app.get_user_by_id(requester.into(), user_id.into())
        .await?
        .pipe(UserDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn update_user_by_id<App>(
    app: State<App>,
    requester: UserSession,
    Path(((), user_id)): Path<((), Uuid)>,
    Json(update): Json<UserUpdateDto>,
) -> ApiResult<impl IntoResponse>
where
    App: UpdateUserByIdUsecase,
{
    let input = {
        let new_is_active = update.is_active.clone();
        let new_role = update.role.clone();
        (
            update.parse().map_err(Into::into),
            new_is_active.into(),
            new_role.into(),
        )
    };

    app.update_user_by_id(requester.into(), user_id.into(), input)
        .await?
        .pipe(UserDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn deactivate_user_by_id<App>(
    app: State<App>,
    requester: AdminSession,
    Path(((), user_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    App: DeactivateUserByIdUsecase,
{
    app.deactivate_user_by_id(requester.into(), user_id.into())
        .await?;

    StatusCode::NO_CONTENT.pipe(Ok)
}
