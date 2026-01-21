use application::usecase::{
    session::SessionUseCase as _, user::UserUseCase as _,
};
use axum::{
    Router, extract::State, http::StatusCode, response::IntoResponse,
    routing::post,
};
use lib::{
    presentation::api::rest::{
        model::Parseable as _, response::ResponseExt as _,
    },
    tap::Pipe as _,
};

use crate::{
    ApiError, ModulesExt,
    extractors::Json,
    models::{
        session::{CreateJsonSession, JsonUserSession},
        user::CreateJsonUser,
    },
};

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new()
        .route("/register", post(register::<M>))
        .route("/login", post(login::<M>))
}

pub async fn register<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateJsonUser>,
) -> Result<impl IntoResponse, ApiError> {
    let user = source.parse();

    let user = modules.user_usecase().create(None, user).await?;

    let token = modules.session_usecase().create(user.id, user.role)?;

    JsonUserSession::from((token, user.into()))
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

pub async fn login<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateJsonSession>,
) -> Result<impl IntoResponse, ApiError> {
    let credentials = source.parse()?;

    let user = modules.user_usecase().authorize(credentials).await?;

    let token = modules.session_usecase().create(user.id, user.role)?;

    JsonUserSession::from((token, user.into()))
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
