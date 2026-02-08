use chrono::{DateTime, Utc};
use domain::statistics::overview::StatsOverview;
use lib::model_mapper::Mapper;
use serde::Serialize;

use crate::dto::statistics::merchants::risk::MerchantRiskStatsDto;

pub mod filter;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = StatsOverview, from)]
#[serde(rename_all = "camelCase")]
pub struct StatsOverviewDto {
    pub from: DateTime<Utc>,

    pub to: DateTime<Utc>,

    pub volume: i64,

    pub gmv: f64,

    pub approval_rate: f32,

    pub decline_rate: f32,

    #[mapper(iter)]
    pub top_risk_merchants: Vec<MerchantRiskStatsDto>,
}
