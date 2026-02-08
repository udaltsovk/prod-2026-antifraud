use chrono::{DateTime, Utc};
use domain::statistics::overview::StatsOverview;
use lib::model_mapper::Mapper;
use sqlx::FromRow;

use crate::entity::statistics::merchant_risk::StoredMerchantRiskStats;

#[derive(Mapper, FromRow)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(derive(ty = StatsOverview, into))]
pub struct StoredStatsOverview {
    pub from: DateTime<Utc>,

    pub to: DateTime<Utc>,

    pub volume: i64,

    pub gmv: f64,

    pub approval_rate: f32,

    pub decline_rate: f32,

    #[mapper(
        when(ty = StatsOverview, iter),
    )]
    pub top_risk_merchants: Vec<StoredMerchantRiskStats>,
}
