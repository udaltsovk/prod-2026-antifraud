use chrono::{DateTime, Utc};
use domain::statistics::transactions::TransactionsTimeseriesPoint;
use lib::model_mapper::Mapper;
use serde::Serialize;

pub mod filter;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = TransactionsTimeseriesPoint, from)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsTimeseriesPointDto {
    pub bucket_start: DateTime<Utc>,

    pub tx_count: i64,

    pub gmv: f64,

    pub approval_rate: f32,

    pub decline_rate: f32,
}
