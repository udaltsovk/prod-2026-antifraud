use application::repository::statistics::StatisticsRepositoryImpl;
use domain::{
    pagination::time_based::TimeBasedPagination,
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
use lib::{
    anyhow::Result,
    application::di::Has,
    async_trait,
    domain::{DomainType, Id},
    infrastructure::persistence::{HasPoolExt as _, sqlx::SqlxPool},
    instrument_all,
    tap::Pipe as _,
};
use sqlx::{Postgres, query_file_as};

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

#[entrait]
#[async_trait]
#[instrument_all]
impl StatisticsRepositoryImpl for PostgresRepositoryImpl {
    async fn statistics_overview<Deps>(
        deps: &Deps,
        StatsOverviewFilter {
            time_based_pagination:
                TimeBasedPagination {
                    from,
                    to,
                },
        }: StatsOverviewFilter,
    ) -> Result<StatsOverview>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let mut connection = deps.get_connection().await?;

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

    async fn statistics_transactions_timeseries<Deps>(
        deps: &Deps,
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
    ) -> Result<Vec<TransactionsTimeseriesPoint>>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let group_by = group_by.to_string();
        let timezone = timezone.into_inner().name();
        let channel = channel.map(StoredTransactionChannel::from);

        let mut connection = deps.get_connection().await?;

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

    async fn statistics_rules_matches<Deps>(
        deps: &Deps,
        RulesMatchesStatsFilter {
            time_based_pagination:
                TimeBasedPagination {
                    from,
                    to,
                },
            top,
        }: RulesMatchesStatsFilter,
    ) -> Result<Vec<RuleMatchesStats>>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let top: i64 = top.into();

        let mut connection = deps.get_connection().await?;

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

    async fn statistics_merchants_risk<Deps>(
        deps: &Deps,
        MerchantsRiskStatsFilter {
            time_based_pagination:
                TimeBasedPagination {
                    from,
                    to,
                },
            merchant_category_code,
            top,
        }: MerchantsRiskStatsFilter,
    ) -> Result<Vec<MerchantRiskStats>>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let merchant_category_code =
            merchant_category_code.map(DomainType::into_inner);
        let top: i64 = top.into();

        let mut connection = deps.get_connection().await?;

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

    async fn statistics_user_risk_profile<Deps>(
        deps: &Deps,
        user_id: Id<User>,
    ) -> Result<UserRiskProfile>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let mut connection = deps.get_connection().await?;

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
