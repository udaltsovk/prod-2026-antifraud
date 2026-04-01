use domain::statistics::overview::{
    StatsOverview, filter::StatsOverviewFilterInput,
};
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    repository::statistics::StatisticsRepository,
    usecase::statistics::StatisticsUseCaseResult,
};

#[entrait(pub StatisticsOverviewUsecase)]
#[instrument(skip(deps))]
async fn statistics_overview<Deps>(
    deps: &Deps,
    filter: StatsOverviewFilterInput,
) -> StatisticsUseCaseResult<StatsOverview>
where
    Deps: StatisticsRepository,
{
    StatisticsRepository::statistics_overview(deps, filter.normalize())
        .await?
        .pipe(Ok)
}
