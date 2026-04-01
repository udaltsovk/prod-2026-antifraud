use application::{
    Application, usecase::statistics::StatisticsRulesMatchesUsecase,
};
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use lib::{
    presentation::api::rest::validation::parseable::Parseable as _,
    tap::Pipe as _,
};
use serde_json::json;

use crate::{
    dto::statistics::rules::matches::{
        RuleMatchesStatsDto, filter::RulesMatchesStatsFilterQuery,
    },
    errors::ApiResult,
    extractors::{Json, Query, session::AdminSession},
};

pub fn router<App>() -> Router<App>
where
    App: Application,
{
    Router::new().route("/matches", get(rules_matches::<App>))
}

pub async fn rules_matches<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
    Query(filter): Query<RulesMatchesStatsFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    App: StatisticsRulesMatchesUsecase,
{
    let filter = filter.parse()?;

    let stats: Vec<_> = app
        .statistics_rules_matches(filter)
        .await?
        .into_iter()
        .map(RuleMatchesStatsDto::from)
        .collect();

    json!({ "items": stats })
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}
