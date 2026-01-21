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
        model::Parseable as _, response::ResponseExt as _,
    },
    tap::{Conv as _, Pipe as _},
};

use crate::{
    ModulesExt,
    errors::ApiError,
    extractors::{Json, Query, session::UserSession},
    models::{
        pagination::{Paginated, QueryPagination},
        user::{CreateJsonUserWithRole, JsonUser},
    },
};

pub mod by_id;
pub mod me;

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new()
        .route("/me", get(me::get_user_curent::<M>))
        .route("/{user_id}", get(by_id::get_user_by_id::<M>))
        .route("/", post(register_user::<M>).get(list_users::<M>))
}

pub async fn register_user<M: ModulesExt>(
    modules: State<M>,
    user_session: UserSession,
    Json(source): Json<CreateJsonUserWithRole>,
) -> Result<impl IntoResponse, ApiError> {
    let source = source.parse();

    modules
        .user_usecase()
        .create(Some(user_session.user_role), source)
        .await?
        .conv::<JsonUser>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

pub async fn list_users<M: ModulesExt>(
    modules: State<M>,
    user_session: UserSession,
    Query(pagination): Query<QueryPagination>,
) -> Result<impl IntoResponse, ApiError> {
    let pagination = pagination.parse();

    let (users, count) = modules
        .user_usecase()
        .list(Some(user_session.user_role), pagination.clone(), None, None)
        .await
        .map_err(ApiError::from)?;

    Paginated::<JsonUser>::from_pagination(pagination?, users, count)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
