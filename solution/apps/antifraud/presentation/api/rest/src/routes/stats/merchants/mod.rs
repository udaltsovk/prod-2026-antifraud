use application::{
    Application, usecase::statistics::StatisticsMerchantsRiskUsecase,
};
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use lib::{
    presentation::api::rest::validation::parseable::Parseable as _,
    tap::Pipe as _,
};
use serde_json::json;

use crate::{
    dto::statistics::merchants::risk::{
        MerchantRiskStatsDto, filter::MerchantsRiskStatsFilterQuery,
    },
    errors::ApiResult,
    extractors::{Json, Query, session::AdminSession},
};

pub fn router<App>() -> Router<App>
where
    App: Application,
{
    Router::new().route("/risk", get(merchants_risk::<App>))
}

pub async fn merchants_risk<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
    Query(filter): Query<MerchantsRiskStatsFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    App: StatisticsMerchantsRiskUsecase,
{
    let filter = filter.parse()?;

    let stats: Vec<_> = app
        .statistics_merchants_risk(filter)
        .await?
        .into_iter()
        .map(MerchantRiskStatsDto::from)
        .collect();

    json!({ "items": stats })
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}
