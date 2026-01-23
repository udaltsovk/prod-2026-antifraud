use axum::{extract::State, response::IntoResponse};

use crate::{
    ModulesExt,
    errors::ApiResult,
    extractors::{Json, Path, session::UserSession},
    models::user::JsonUserUpdate,
    routes::users::by_id,
};

pub async fn get_current_user<M>(
    modules: State<M>,
    user_session: UserSession,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let user_id = Path(((), user_session.user_id.value));
    by_id::get_user_by_id(modules, user_session, user_id).await
}

pub async fn update_current_user<M>(
    modules: State<M>,
    user_session: UserSession,
    update: Json<JsonUserUpdate>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let user_id = Path(((), user_session.user_id.value));
    by_id::update_user_by_id(modules, user_session, user_id, update).await
}
