use application::usecase::{
    session::SessionUseCase as _,
    user::{CreateUserSource, UserUseCase as _},
};
use axum::{
    Router, extract::State, http::StatusCode, response::IntoResponse,
    routing::post,
};
use lib::{
    presentation::api::rest::{
        response::ResponseExt as _, validation::parseable::Parseable as _,
    },
    tap::Pipe as _,
};

use crate::{
    ModulesExt,
    dto::{
        session::{CreateSessionDto, UserSessionDto},
        user::CreateUserDto,
    },
    errors::ApiResult,
    extractors::Json,
};

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new()
        .route("/register", post(register::<M>))
        .route("/login", post(login::<M>))
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn register<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateUserDto>,
) -> ApiResult<impl IntoResponse> {
    let user = source.parse().map_err(Into::into);

    let user = modules
        .user_usecase()
        .create(CreateUserSource::Registration, user)
        .await?;

    let token = modules.session_usecase().create(user.id, user.role)?;

    UserSessionDto::from((token, user.into()))
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn login<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateSessionDto>,
) -> ApiResult<impl IntoResponse> {
    let credentials = source.parse()?;

    let user = modules.user_usecase().authorize(credentials).await?;

    let token = modules.session_usecase().create(user.id, user.role)?;

    UserSessionDto::from((token, user.into()))
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
