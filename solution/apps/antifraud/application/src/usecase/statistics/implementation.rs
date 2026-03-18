use domain::{
    statistics::{
        Statistics,
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
use lib::{async_trait, domain::Id, instrument_all, tap::Pipe as _};

use crate::usecase::{
    UseCase,
    statistics::{
        StatisticsUseCase,
        error::{StatisticsUseCaseError, StatisticsUseCaseResult},
    },
};

#[async_trait]
#[instrument_all]
impl StatisticsUseCase for UseCase<Statistics> {
    async fn overview(
        &self,
        filter: StatsOverviewFilterInput,
    ) -> StatisticsUseCaseResult<StatsOverview> {
        let filter = filter.normalize();

        self.repositories
            .statistics()
            .overview(filter)
            .await
            .map_err(StatisticsUseCaseError::Infrastructure)
    }

    async fn transactions_timeseries(
        &self,
        filter: TransactionsTimeseriesPointFilterInput,
    ) -> StatisticsUseCaseResult<Vec<TransactionsTimeseriesPoint>> {
        let filter = filter.normalize();

        self.repositories
            .statistics()
            .transactions_timeseries(filter)
            .await
            .map_err(StatisticsUseCaseError::Infrastructure)
    }

    async fn rules_matches(
        &self,
        filter: RulesMatchesStatsFilterInput,
    ) -> StatisticsUseCaseResult<Vec<RuleMatchesStats>> {
        let filter = filter.normalize();

        self.repositories
            .statistics()
            .rules_matches(filter)
            .await
            .map_err(StatisticsUseCaseError::Infrastructure)
    }

    async fn merchants_risk(
        &self,
        filter: MerchantsRiskStatsFilterInput,
    ) -> StatisticsUseCaseResult<Vec<MerchantRiskStats>> {
        let filter = filter.normalize();

        self.repositories
            .statistics()
            .merchants_risk(filter)
            .await
            .map_err(StatisticsUseCaseError::Infrastructure)
    }

    async fn user_risk_profile(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> StatisticsUseCaseResult<UserRiskProfile> {
        if requester_role != UserRole::Admin && user_id != requester_id {
            return StatisticsUseCaseError::MissingPermissions.pipe(Err);
        }

        self.repositories
            .user()
            .find_by_id(user_id)
            .await
            .map_err(StatisticsUseCaseError::Infrastructure)?
            .ok_or(StatisticsUseCaseError::UserNotFoundById(user_id))?;

        let mut risk_profile = self
            .repositories
            .statistics()
            .user_risk_profile(user_id)
            .await
            .map_err(StatisticsUseCaseError::Infrastructure)?;

        let user_activity = self
            .repositories
            .user_activity()
            .find_by_user(user_id)
            .await
            .map_err(StatisticsUseCaseError::Infrastructure)?;

        risk_profile.last_seen_at = user_activity;

        risk_profile.pipe(Ok)
    }
}
