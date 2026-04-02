use domain::{
    statistics::{
        merchants::{MerchantRiskStats, filter::MerchantsRiskStatsFilter},
        overview::{StatsOverview, filter::StatsOverviewFilter},
        rules::{RuleMatchesStats, filter::RulesMatchesStatsFilter},
        transactions::{
            TransactionsTimeseriesPoint,
            filter::TransactionsTimeseriesPointFilter,
        },
        users::UserRiskProfile,
    },
    user::User,
};
use entrait::entrait;
use lib::{anyhow::Result, async_trait, domain::Id};

#[entrait(
    StatisticsRepositoryImpl,
    delegate_by=DelegateStatisticsRepository
)]
#[async_trait]
pub trait StatisticsRepository {
    async fn statistics_overview(
        &self,
        filter: StatsOverviewFilter,
    ) -> Result<StatsOverview>;

    async fn statistics_transactions_timeseries(
        &self,
        filter: TransactionsTimeseriesPointFilter,
    ) -> Result<Vec<TransactionsTimeseriesPoint>>;

    async fn statistics_rules_matches(
        &self,
        filter: RulesMatchesStatsFilter,
    ) -> Result<Vec<RuleMatchesStats>>;

    async fn statistics_merchants_risk(
        &self,
        filter: MerchantsRiskStatsFilter,
    ) -> Result<Vec<MerchantRiskStats>>;

    async fn statistics_user_risk_profile(
        &self,
        user_id: Id<User>,
    ) -> Result<UserRiskProfile>;
}
