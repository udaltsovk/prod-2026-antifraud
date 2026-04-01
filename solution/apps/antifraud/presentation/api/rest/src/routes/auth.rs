use application::{
    Application,
    usecase::{
        session::CreateSessionUsecase,
        user::{AuthorizeUserUsecase, CreateUserSource, CreateUserUsecase},
    },
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
    dto::{
        session::{CreateSessionDto, UserSessionDto},
        user::CreateUserDto,
    },
    errors::ApiResult,
    extractors::Json,
};

pub fn router<App>() -> Router<App>
where
    App: Application,
{
    Router::new()
        .route("/register", post(register::<App>))
        .route("/login", post(login::<App>))
}

pub async fn register<App>(
    app: State<App>,
    Json(source): Json<CreateUserDto>,
) -> ApiResult<impl IntoResponse>
where
    App: CreateUserUsecase + CreateSessionUsecase,
{
    let user = source.parse().map_err(Into::into);

    let user = app
        .create_user(CreateUserSource::Registration, user)
        .await?;

    let token = app.create_session(user.id, user.role)?;

    UserSessionDto::from((token, user.into()))
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

pub async fn login<App>(
    app: State<App>,
    Json(source): Json<CreateSessionDto>,
) -> ApiResult<impl IntoResponse>
where
    App: AuthorizeUserUsecase + CreateSessionUsecase,
{
    let credentials = source.parse()?;

    let user = app.authorize_user(credentials).await?;

    let token = app.create_session(user.id, user.role)?;

    UserSessionDto::from((token, user.into()))
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
