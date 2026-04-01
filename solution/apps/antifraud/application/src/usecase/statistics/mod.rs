use domain::user::User;
use lib::{
    application::application_result,
    domain::{Id, validation::error::ValidationErrorsWithFields},
};

mod merchants_risk;
mod overview;
mod rules_matches;
mod transactions_timeseries;
mod user_risk_profile;

pub use merchants_risk::StatisticsMerchantsRiskUsecase;
pub use overview::StatisticsOverviewUsecase;
pub use rules_matches::StatisticsRulesMatchesUsecase;
pub use transactions_timeseries::StatisticsTransactionsTimeseriesUsecase;
pub use user_risk_profile::StatisticsUserRiskProfileUsecase;

#[derive(thiserror::Error, Debug)]
pub enum StatisticsUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error(transparent)]
    Validation(ValidationErrorsWithFields),

    #[error("Пользователь не найден")]
    UserNotFoundById(Id<User>),

    #[error("Недостаточно прав для выполнения операции")]
    MissingPermissions,
}

application_result!(StatisticsUseCase);
