use application::usecase::user::UserUseCase as _;
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
    ModulesExt,
    dto::{
        pagination::{Paginated, QueryPagination},
        user::{CreateUserWithRoleDto, UserDto},
    },
    errors::ApiResult,
    extractors::{Json, Query, session::AdminSession},
};

pub mod by_id;
pub mod me;

pub fn router<M>() -> Router<M>
where
    M: ModulesExt,
{
    Router::new()
        .route(
            "/me",
            get(me::get_current_user::<M>).put(me::update_current_user::<M>),
        )
        .route(
            "/{user_id}",
            get(by_id::get_user_by_id::<M>)
                .put(by_id::update_user_by_id::<M>)
                .delete(by_id::deactivate_user_by_id::<M>),
        )
        .route("/", post(register_user::<M>).get(list_users::<M>))
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn register_user<M>(
    modules: State<M>,
    AdminSession {
        user_role: creator_role,
        ..
    }: AdminSession,
    Json(source): Json<CreateUserWithRoleDto>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let source = source.parse().map_err(Into::into);

    modules
        .user_usecase()
        .create(creator_role.into(), source)
        .await
        .map(UserDto::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn list_users<M>(
    modules: State<M>,
    AdminSession {
        user_role: requester_role,
        ..
    }: AdminSession,
    Query(pagination): Query<QueryPagination>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let pagination = pagination.parse().map_err(Into::into);

    let (users, count) = modules
        .user_usecase()
        .list(requester_role, pagination.clone())
        .await?;

    Paginated::<UserDto>::from_pagination(pagination?, users, count)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
