use application::usecase::user::{GetUserByIdUsecase, UpdateUserByIdUsecase};
use axum::{extract::State, response::IntoResponse};

use crate::{
    dto::user::UserUpdateDto,
    errors::ApiResult,
    extractors::{Json, Path, session::UserSession},
    routes::users::by_id,
};

pub async fn get_current_user<App>(
    app: State<App>,
    user_session: UserSession,
) -> ApiResult<impl IntoResponse>
where
    App: GetUserByIdUsecase,
{
    let user_id = Path(((), user_session.user_id.value));
    by_id::get_user_by_id(app, user_session, user_id).await
}

pub async fn update_current_user<App>(
    app: State<App>,
    user_session: UserSession,
    update: Json<UserUpdateDto>,
) -> ApiResult<impl IntoResponse>
where
    App: UpdateUserByIdUsecase,
{
    let user_id = Path(((), user_session.user_id.value));
    by_id::update_user_by_id(app, user_session, user_id, update).await
}
