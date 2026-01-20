use application::usecase::{
    session::SessionUseCase as _, user::UserUseCase as _,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use domain::session::entity::SessionEntity;
use lib::{
    presentation::api::rest::{
        model::ParseableJson as _, response::ResponseExt as _,
    },
    tap::{Conv as _, Pipe as _},
};

use crate::{
    ApiError, ModulesExt,
    extractors::Json,
    models::{
        session::{CreateJsonSession, JsonUserSession},
        user::CreateJsonUser,
    },
};

pub async fn sign_up<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateJsonUser>,
) -> Result<impl IntoResponse, ApiError> {
    let user = source.parse()?;

    let user = modules.user_usecase().create(user).await?;

    modules
        .session_usecase()
        .create(SessionEntity::from(&user))
        .await?
        .conv::<JsonUserSession>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn sign_in<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateJsonSession>,
) -> Result<impl IntoResponse, ApiError> {
    let credentials = source.parse()?;

    let user = modules.user_usecase().authorize(credentials).await?;

    modules
        .session_usecase()
        .create(SessionEntity::from(&user))
        .await?
        .conv::<JsonUserSession>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
