use application::usecase::statistics::StatisticsUseCase as _;
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use lib::{
    presentation::api::rest::validation::parseable::Parseable as _,
    tap::Pipe as _,
};

use crate::{
    ModulesExt,
    dto::statistics::overview::{
        StatsOverviewDto, filter::StatsOverviewFilterQuery,
    },
    errors::ApiResult,
    extractors::{Json, Query, session::AdminSession},
};

pub mod merchants;
pub mod rules;
pub mod transactions;
pub mod users;

pub fn router<M: ModulesExt>() -> Router<M> {
    Router::new()
        .route("/overview", get(stats_overview::<M>))
        .nest("/transactions", transactions::router())
        .nest("/rules", rules::router())
        .nest("/merchants", merchants::router())
        .nest("/users", users::router())
}

pub async fn stats_overview<M>(
    modules: State<M>,
    AdminSession {
        ..
    }: AdminSession,
    Query(filter): Query<StatsOverviewFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let filter = filter.parse()?;

    modules
        .statistics_usecase()
        .overview(filter)
        .await?
        .pipe(StatsOverviewDto::from)
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}
