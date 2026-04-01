use application::{
    Application,
    usecase::transaction::{
        BulkCreateTransactionsUsecase, CreateTransactionUsecase,
        GetTransactionByIdUsecase, ListTransactionsUsecase,
    },
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
        errors::JsonError, response::ResponseExt as _,
        validation::parseable::Parseable as _,
    },
    tap::Pipe as _,
    uuid::Uuid,
};
use serde_json::json;

use crate::{
    dto::{
        pagination::Paginated,
        transaction::{
            BulkCreateTransactionsDto, BulkTransactionDto,
            CreateTransactionDto, TransactionDto,
            decision::TransactionDecisionDto, filter::TransactionFilterQuery,
        },
    },
    errors::{ApiError, ApiResult},
    extractors::{Json, Path, Query, session::UserSession},
};

pub fn router<App>() -> Router<App>
where
    App: Application,
{
    Router::new()
        .route(
            "/",
            post(create_transaction::<App>).get(list_transactions::<App>),
        )
        .route("/{transaction_id}", get(get_transaction_by_id::<App>))
        .route("/batch", post(bulk_create_transactions::<App>))
}

pub async fn create_transaction<App>(
    app: State<App>,
    creator: UserSession,
    Json(input): Json<CreateTransactionDto>,
) -> ApiResult<impl IntoResponse>
where
    App: CreateTransactionUsecase,
{
    let input = {
        let transaction_user_id = input.user_id.clone();
        (
            input.parse().map_err(Into::into),
            transaction_user_id.into(),
        )
    };

    app.create_transaction(creator.into(), input)
        .await?
        .pipe(TransactionDecisionDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::CREATED)
        .pipe(Ok)
}

pub async fn list_transactions<App>(
    app: State<App>,
    requester: UserSession,
    Query(filter): Query<TransactionFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    App: ListTransactionsUsecase,
{
    let input = {
        let user_id = filter.user_id.clone();
        (filter.parse().map_err(Into::into), user_id.into())
    };

    let (transactions, count) = app
        .list_transactions(requester.into(), input.clone())
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

pub async fn get_transaction_by_id<App>(
    app: State<App>,
    requester: UserSession,
    Path(((), transaction_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    App: GetTransactionByIdUsecase,
{
    app.get_transaction_by_id(requester.into(), transaction_id.into())
        .await?
        .pipe(TransactionDecisionDto::from)
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}

pub async fn bulk_create_transactions<App>(
    app: State<App>,
    creator: UserSession,
    Json(input): Json<BulkCreateTransactionsDto>,
) -> ApiResult<impl IntoResponse>
where
    App: BulkCreateTransactionsUsecase,
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

    let items: Vec<_> = app
        .bulk_create_transactions(creator.into(), input)
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
