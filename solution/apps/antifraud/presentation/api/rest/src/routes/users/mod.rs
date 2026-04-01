use application::{
    Application,
    usecase::user::{CreateUserUsecase, ListUsersUsecase},
};
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use lib::{
    presentation::api::rest::{
        response::ResponseExt as _, validation::parseable::Parseable as _,
    },
    tap::Pipe as _,
};

use crate::{
    dto::{
        pagination::Paginated,
        user::{CreateUserWithRoleDto, UserDto, filter::UserFilterQuery},
    },
    errors::ApiResult,
    extractors::{Json, Query, session::AdminSession},
};

pub mod by_id;
pub mod me;

pub fn router<App>() -> Router<App>
where
    App: Application,
{
    Router::new()
        .route(
            "/me",
            get(me::get_current_user::<App>)
                .put(me::update_current_user::<App>),
        )
        .route(
            "/{user_id}",
            get(by_id::get_user_by_id::<App>)
                .put(by_id::update_user_by_id::<App>)
                .delete(by_id::deactivate_user_by_id::<App>),
        )
        .route("/", post(register_user::<App>).get(list_users::<App>))
}

pub async fn register_user<App>(
    app: State<App>,
    AdminSession {
        user_role: creator_role,
        ..
    }: AdminSession,
    Json(source): Json<CreateUserWithRoleDto>,
) -> ApiResult<impl IntoResponse>
where
    App: CreateUserUsecase,
{
    let source = source.parse().map_err(Into::into);

    app.create_user(creator_role.into(), source)
        .await?
        .pipe(UserDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

pub async fn list_users<App>(
    app: State<App>,
    AdminSession {
        user_role: requester_role,
        ..
    }: AdminSession,
    Query(filter): Query<UserFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    App: ListUsersUsecase,
{
    let input = filter.parse().map_err(Into::into);

    let (users, count) = app.list_users(requester_role, input.clone()).await?;

    Paginated::<UserDto>::from_pagination(input?.pagination, users, count)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
