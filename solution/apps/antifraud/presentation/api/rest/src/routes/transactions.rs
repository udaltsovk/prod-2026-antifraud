use application::usecase::transaction::TransactionUseCase as _;
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
    uuid::Uuid,
};

use crate::{
    ModulesExt,
    dto::{
        pagination::Paginated,
        transaction::{
            CreateTransactionDto, TransactionDto,
            decision::TransactionDecisionDto,
            pagination::QueryTransactionPagination,
        },
    },
    errors::ApiResult,
    extractors::{Json, Path, Query, session::UserSession},
};

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new()
        .route(
            "/",
            post(create_transaction::<M>).get(list_transactions::<M>),
        )
        .route("/{transaction_id}", get(get_transaction_by_id::<M>))
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn create_transaction<M>(
    modules: State<M>,
    creator: UserSession,
    Json(input): Json<CreateTransactionDto>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let input = {
        let transaction_user_id = input.user_id.clone();
        (
            input.parse().map_err(Into::into),
            transaction_user_id.into(),
        )
    };

    modules
        .transaction_usecase()
        .create(creator.into(), input)
        .await
        .map(TransactionDecisionDto::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn list_transactions<M>(
    modules: State<M>,
    requester: UserSession,
    Query(pagination): Query<QueryTransactionPagination>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let input = {
        let user_id = pagination.user_id.clone();
        (pagination.parse().map_err(Into::into), user_id.into())
    };

    let (transactions, count) = modules
        .transaction_usecase()
        .list(requester.into(), input.clone())
        .await?;

    Paginated::<TransactionDto>::from_pagination(
        input.0?.pagination,
        transactions,
        count,
    )
    .pipe(Json)
    .into_response()
    .with_status(StatusCode::OK)
    .pipe(Ok)
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn get_transaction_by_id<M>(
    modules: State<M>,
    requester: UserSession,
    Path(((), transaction_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .transaction_usecase()
        .get_by_id(requester.into(), transaction_id.into())
        .await
        .map(TransactionDecisionDto::from)
        .map(Json)?
        .into_response()
        .pipe(Ok)
}
