use domain::{
    statistics::{
        merchants::{MerchantRiskStats, filter::MerchantsRiskStatsFilterInput},
        overview::{StatsOverview, filter::StatsOverviewFilterInput},
        rules::{RuleMatchesStats, filter::RulesMatchesStatsFilterInput},
        transactions::{
            TransactionsTimeseriesPoint,
            filter::TransactionsTimeseriesPointFilterInput,
        },
        users::UserRiskProfile,
    },
    user::{User, role::UserRole},
};
use lib::{async_trait, domain::Id};

use crate::usecase::statistics::error::StatisticsUseCaseResult;

pub mod error;
pub mod implementation;

#[async_trait]
pub trait StatisticsUseCase {
    async fn overview(
        &self,
        filter: StatsOverviewFilterInput,
    ) -> StatisticsUseCaseResult<StatsOverview>;

    async fn transactions_timeseries(
        &self,
        filter: TransactionsTimeseriesPointFilterInput,
    ) -> StatisticsUseCaseResult<Vec<TransactionsTimeseriesPoint>>;

    async fn rules_matches(
        &self,
        filter: RulesMatchesStatsFilterInput,
    ) -> StatisticsUseCaseResult<Vec<RuleMatchesStats>>;

    async fn merchants_risk(
        &self,
        filter: MerchantsRiskStatsFilterInput,
    ) -> StatisticsUseCaseResult<Vec<MerchantRiskStats>>;

    async fn user_risk_profile(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> StatisticsUseCaseResult<UserRiskProfile>;
}
