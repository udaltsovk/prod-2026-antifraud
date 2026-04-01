use domain::statistics::merchants::{
    MerchantRiskStats, filter::MerchantsRiskStatsFilterInput,
};
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    repository::statistics::StatisticsRepository,
    usecase::statistics::StatisticsUseCaseResult,
};

#[entrait(pub StatisticsMerchantsRiskUsecase)]
#[instrument(skip(deps))]
async fn statistics_merchants_risk<Deps>(
    deps: &Deps,
    filter: MerchantsRiskStatsFilterInput,
) -> StatisticsUseCaseResult<Vec<MerchantRiskStats>>
where
    Deps: StatisticsRepository,
{
    StatisticsRepository::statistics_merchants_risk(deps, filter.normalize())
        .await?
        .pipe(Ok)
}
