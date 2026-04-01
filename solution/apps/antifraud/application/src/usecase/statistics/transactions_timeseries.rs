use domain::statistics::transactions::{
    TransactionsTimeseriesPoint, filter::TransactionsTimeseriesPointFilterInput,
};
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    repository::statistics::StatisticsRepository,
    usecase::statistics::StatisticsUseCaseResult,
};

#[entrait(pub StatisticsTransactionsTimeseriesUsecase)]
#[instrument(skip(deps))]
async fn statistics_transactions_timeseries<Deps>(
    deps: &Deps,
    filter: TransactionsTimeseriesPointFilterInput,
) -> StatisticsUseCaseResult<Vec<TransactionsTimeseriesPoint>>
where
    Deps: StatisticsRepository,
{
    StatisticsRepository::statistics_transactions_timeseries(
        deps,
        filter.normalize(),
    )
    .await?
    .pipe(Ok)
}
