use application::usecase::statistics::StatisticsUseCase as _;
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use lib::{
    presentation::api::rest::validation::parseable::Parseable as _,
    tap::Pipe as _,
};
use serde_json::json;

use crate::{
    ModulesExt,
    dto::statistics::merchants::risk::{
        MerchantRiskStatsDto, filter::MerchantsRiskStatsFilterQuery,
    },
    errors::ApiResult,
    extractors::{Json, Query, session::AdminSession},
};

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new().route("/risk", get(merchants_risk::<M>))
}

pub async fn merchants_risk<M>(
    modules: State<M>,
    AdminSession {
        ..
    }: AdminSession,
    Query(filter): Query<MerchantsRiskStatsFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let filter = filter.parse()?;

    let stats: Vec<_> = modules
        .statistics_usecase()
        .merchants_risk(filter)
        .await?
        .into_iter()
        .map(MerchantRiskStatsDto::from)
        .collect();

    json!({ "items": stats })
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}
