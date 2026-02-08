use chrono::{DateTime, Utc};

use crate::statistics::merchants::MerchantRiskStats;

pub mod filter;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StatsOverview {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub volume: i64,
    pub gmv: f64,
    pub approval_rate: f32,
    pub decline_rate: f32,
    pub top_risk_merchants: Vec<MerchantRiskStats>,
}
