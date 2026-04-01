use domain::statistics::rules::{
    RuleMatchesStats, filter::RulesMatchesStatsFilterInput,
};
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    repository::statistics::StatisticsRepository,
    usecase::statistics::StatisticsUseCaseResult,
};

#[entrait(pub StatisticsRulesMatchesUsecase)]
#[instrument(skip(deps))]
async fn statistics_rules_matches<Deps>(
    deps: &Deps,
    filter: RulesMatchesStatsFilterInput,
) -> StatisticsUseCaseResult<Vec<RuleMatchesStats>>
where
    Deps: StatisticsRepository,
{
    StatisticsRepository::statistics_rules_matches(deps, filter.normalize())
        .await?
        .pipe(Ok)
}
