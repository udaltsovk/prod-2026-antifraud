use application::{
    Application, usecase::statistics::StatisticsOverviewUsecase,
};
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use lib::{
    presentation::api::rest::validation::parseable::Parseable as _,
    tap::Pipe as _,
};

use crate::{
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

pub fn router<App>() -> Router<App>
where
    App: Application,
{
    Router::new()
        .route("/overview", get(stats_overview::<App>))
        .nest("/transactions", transactions::router())
        .nest("/rules", rules::router())
        .nest("/merchants", merchants::router())
        .nest("/users", users::router())
}

pub async fn stats_overview<App>(
    app: State<App>,
    AdminSession {
        ..
    }: AdminSession,
    Query(filter): Query<StatsOverviewFilterQuery>,
) -> ApiResult<impl IntoResponse>
where
    App: StatisticsOverviewUsecase,
{
    let filter = filter.parse()?;

    app.statistics_overview(filter)
        .await?
        .pipe(StatsOverviewDto::from)
        .pipe(Json)
        .into_response()
        .pipe(Ok)
}
