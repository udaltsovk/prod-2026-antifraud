use application::{
    Application, usecase::statistics::StatisticsTransactionsTimeseriesUsecase,
};
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use lib::{
    presentation::api::rest::validation::parseable::Parseable as _,
    tap::Pipe as _,
};
use serde_json::json;

use crate::{
    dto::statistics::transactions::timeseries::{
        TransactionsTimeseriesPointDto,
        filter::TransactionsTimeseriesPointFilterQuery,
    },
    errors::ApiResult,
    extractors::{Json, Query, session::AdminSession},
};

pub fn router<App>() -> Router<App>
where
    App: Application,
{
    Router::new().route("/timeseries", get(transactions_timeseries::<App>))
}

pub async fn transactions_timeseries<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
    Query(filter): Query<TransactionsTimeseriesPointFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    App: StatisticsTransactionsTimeseriesUsecase,
{
    let filter = filter.parse()?;

    let points: Vec<_> = app
        .statistics_transactions_timeseries(filter)
        .await?
        .into_iter()
        .map(TransactionsTimeseriesPointDto::from)
        .collect();

    json!({ "points": points })
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}
