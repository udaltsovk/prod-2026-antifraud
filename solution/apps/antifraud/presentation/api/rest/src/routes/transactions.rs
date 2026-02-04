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
        errors::JsonError, response::ResponseExt as _,
        validation::parseable::Parseable as _,
    },
    tap::Pipe as _,
    uuid::Uuid,
};
use serde_json::json;

use crate::{
    ModulesExt,
    dto::{
        pagination::Paginated,
        transaction::{
            BulkCreateTransactionsDto, BulkTransactionDto,
            CreateTransactionDto, TransactionDto,
            decision::TransactionDecisionDto,
            pagination::QueryTransactionPagination,
        },
    },
    errors::{ApiError, ApiResult},
    extractors::{Json, Path, Query, session::UserSession},
};

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new()
        .route(
            "/",
            post(create_transaction::<M>).get(list_transactions::<M>),
        )
        .route("/{transaction_id}", get(get_transaction_by_id::<M>))
        .route("/batch", post(bulk_create_transactions::<M>))
}

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

pub async fn bulk_create_transactions<M>(
    modules: State<M>,
    creator: UserSession,
    Json(input): Json<BulkCreateTransactionsDto>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let input: Vec<_> = input
        .parse()?
        .items
        .into_iter()
        .map(|input| {
            let transaction_user_id = input.user_id.clone();
            (
                input.parse().map_err(Into::into),
                transaction_user_id.into(),
            )
        })
        .collect();

    let mut success = true;

    let items: Vec<_> = modules
        .transaction_usecase()
        .bulk_create(creator.into(), input)
        .await?
        .into_iter()
        .map(|(index, result)| {
            success &= result.is_ok();
            let (decision, error) = match result {
                Ok(decision) => (
                    decision.pipe(TransactionDecisionDto::from).pipe(Some),
                    None,
                ),
                Err(error) => (
                    None,
                    error.pipe(ApiError::from).pipe(JsonError::from).pipe(Some),
                ),
            };
            BulkTransactionDto {
                index,
                decision,
                error,
            }
        })
        .collect();

    let status = if success {
        StatusCode::CREATED
    } else {
        StatusCode::MULTI_STATUS
    };

    json!({"items": items})
        .pipe(Json)
        .into_response()
        .with_status(status)
        .pipe(Ok)
}
