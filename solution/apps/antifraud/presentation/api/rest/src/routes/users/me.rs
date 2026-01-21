use axum::{extract::State, response::IntoResponse};

use crate::{
    ModulesExt,
    errors::ApiError,
    extractors::{Path, session::UserSession},
    routes::users::by_id::get_user_by_id,
};

pub async fn get_user_curent<M: ModulesExt>(
    modules: State<M>,
    user_session: UserSession,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = Path(user_session.user_id.value);
    get_user_by_id(modules, user_session, user_id).await
}
