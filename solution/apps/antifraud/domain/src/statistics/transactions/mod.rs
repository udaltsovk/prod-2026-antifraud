use chrono::{DateTime, Utc};

pub mod filter;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionsTimeseriesPoint {
    pub bucket_start: DateTime<Utc>,
    pub tx_count: i64,
    pub gmv: f64,
    pub approval_rate: f32,
    pub decline_rate: f32,
}
