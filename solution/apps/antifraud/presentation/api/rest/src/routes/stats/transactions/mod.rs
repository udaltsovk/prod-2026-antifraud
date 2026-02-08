use application::usecase::statistics::StatisticsUseCase as _;
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use lib::{
    presentation::api::rest::validation::parseable::Parseable as _,
    tap::Pipe as _,
};
use serde_json::json;

use crate::{
    ModulesExt,
    dto::statistics::transactions::timeseries::{
        TransactionsTimeseriesPointDto,
        filter::TransactionsTimeseriesPointFilterQuery,
    },
    errors::ApiResult,
    extractors::{Json, Query, session::AdminSession},
};

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new().route("/timeseries", get(transactions_timeseries::<M>))
}

pub async fn transactions_timeseries<M>(
    modules: State<M>,
    AdminSession {
        ..
    }: AdminSession,
    Query(filter): Query<TransactionsTimeseriesPointFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let filter = filter.parse()?;

    let points: Vec<_> = modules
        .statistics_usecase()
        .transactions_timeseries(filter)
        .await?
        .into_iter()
        .map(TransactionsTimeseriesPointDto::from)
        .collect();

    json!({ "points": points })
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}
