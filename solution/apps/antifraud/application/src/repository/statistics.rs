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
use lib::{anyhow::Result, async_trait, domain::Id};

#[async_trait]
pub trait StatisticsRepository {
    async fn overview(
        &self,
        filter: StatsOverviewFilter,
    ) -> Result<StatsOverview>;

    async fn transactions_timeseries(
        &self,
        filter: TransactionsTimeseriesPointFilter,
    ) -> Result<Vec<TransactionsTimeseriesPoint>>;

    async fn rules_matches(
        &self,
        filter: RulesMatchesStatsFilter,
    ) -> Result<Vec<RuleMatchesStats>>;

    async fn merchants_risk(
        &self,
        filter: MerchantsRiskStatsFilter,
    ) -> Result<Vec<MerchantRiskStats>>;

    async fn user_risk_profile(
        &self,
        user_id: Id<User>,
    ) -> Result<UserRiskProfile>;
}
