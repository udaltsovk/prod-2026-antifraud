use axum::{extract::State, response::IntoResponse};

use crate::{
    ModulesExt,
    errors::ApiResult,
    extractors::{Path, session::UserSession},
    routes::users::by_id,
};

pub async fn get_user_curent<M>(
    modules: State<M>,
    user_session: UserSession,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let user_id = Path(((), user_session.user_id.value));
    by_id::get_user_by_id(modules, user_session, user_id).await
}
