use application::repository::statistics::StatisticsRepository;
use domain::{
    pagination::time_based::TimeBasedPagination,
    statistics::{
        Statistics,
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
use lib::{
    anyhow::Result,
    async_trait,
    domain::{DomainType, Id},
    instrument_all,
    tap::Pipe as _,
};
use sqlx::query_file_as;

use crate::{
    entity::{
        statistics::{
            merchant_risk::StoredMerchantRiskStats,
            overview::StoredStatsOverview,
            rule_matches::StoredRuleMatchesStats,
            transaction_timeseries::StoredTransactionsTimeseriesPoint,
            user_risk_profile::StoredUserRiskProfile,
        },
        transaction::channel::StoredTransactionChannel,
    },
    repository::PostgresRepositoryImpl,
};

#[async_trait]
#[instrument_all]
impl StatisticsRepository for PostgresRepositoryImpl<Statistics> {
    async fn overview(
        &self,
        StatsOverviewFilter {
            time_based_pagination:
                TimeBasedPagination {
                    from,
                    to,
                },
        }: StatsOverviewFilter,
    ) -> Result<StatsOverview> {
        let mut connection = self.pool.get().await?;

        sqlx::query_file_as!(
            StoredStatsOverview,
            "sql/statistics/overview.sql",
            from,
            to,
        )
        .fetch_one(&mut *connection)
        .await?
        .pipe(StatsOverview::from)
        .pipe(Ok)
    }

    async fn transactions_timeseries(
        &self,
        TransactionsTimeseriesPointFilter {
            time_based_pagination:
                TimeBasedPagination {
                    from,
                    to,
                },
            group_by,
            timezone,
            channel,
        }: TransactionsTimeseriesPointFilter,
    ) -> Result<Vec<TransactionsTimeseriesPoint>> {
        let group_by = group_by.to_string();
        let timezone = timezone.into_inner().name();
        let channel = channel.map(StoredTransactionChannel::from);

        let mut connection = self.pool.get().await?;

        query_file_as!(
            StoredTransactionsTimeseriesPoint,
            "sql/statistics/transactions_timeseries.sql",
            from,
            to,
            timezone,
            channel as _,
            group_by,
        )
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(TransactionsTimeseriesPoint::from)
        .collect::<Vec<_>>()
        .pipe(Ok)
    }

    async fn rules_matches(
        &self,
        RulesMatchesStatsFilter {
            time_based_pagination:
                TimeBasedPagination {
                    from,
                    to,
                },
            top,
        }: RulesMatchesStatsFilter,
    ) -> Result<Vec<RuleMatchesStats>> {
        let top: i64 = top.into();

        let mut connection = self.pool.get().await?;

        sqlx::query_file_as!(
            StoredRuleMatchesStats,
            "sql/statistics/rules_matches.sql",
            from,
            to,
            top
        )
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(RuleMatchesStats::from)
        .collect::<Vec<_>>()
        .pipe(Ok)
    }

    async fn merchants_risk(
        &self,
        MerchantsRiskStatsFilter {
            time_based_pagination:
                TimeBasedPagination {
                    from,
                    to,
                },
            merchant_category_code,
            top,
        }: MerchantsRiskStatsFilter,
    ) -> Result<Vec<MerchantRiskStats>> {
        let merchant_category_code =
            merchant_category_code.map(DomainType::into_inner);
        let top: i64 = top.into();

        let mut connection = self.pool.get().await?;

        query_file_as!(
            StoredMerchantRiskStats,
            "sql/statistics/merchants_risk.sql",
            from,
            to,
            merchant_category_code,
            top,
        )
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(MerchantRiskStats::from)
        .collect::<Vec<_>>()
        .pipe(Ok)
    }

    async fn user_risk_profile(
        &self,
        user_id: Id<User>,
    ) -> Result<UserRiskProfile> {
        let mut connection = self.pool.get().await?;

        sqlx::query_file_as!(
            StoredUserRiskProfile,
            "sql/statistics/user_risk_profile.sql",
            user_id.value
        )
        .fetch_one(&mut *connection)
        .await?
        .pipe(UserRiskProfile::from)
        .pipe(Ok)
    }
}
