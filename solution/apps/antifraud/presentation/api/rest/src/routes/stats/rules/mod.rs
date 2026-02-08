use application::usecase::statistics::StatisticsUseCase as _;
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use lib::{
    presentation::api::rest::validation::parseable::Parseable as _,
    tap::Pipe as _,
};
use serde_json::json;

use crate::{
    ModulesExt,
    dto::statistics::rules::matches::{
        RuleMatchesStatsDto, filter::RulesMatchesStatsFilterQuery,
    },
    errors::ApiResult,
    extractors::{Json, Query, session::AdminSession},
};

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new().route("/matches", get(rules_matches::<M>))
}

pub async fn rules_matches<M>(
    modules: State<M>,
    AdminSession {
        ..
    }: AdminSession,
    Query(filter): Query<RulesMatchesStatsFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let filter = filter.parse()?;

    let stats: Vec<_> = modules
        .statistics_usecase()
        .rules_matches(filter)
        .await?
        .into_iter()
        .map(RuleMatchesStatsDto::from)
        .collect();

    json!({ "items": stats })
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}
