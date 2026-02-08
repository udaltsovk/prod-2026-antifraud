use chrono::{DateTime, Utc};
use domain::statistics::transactions::TransactionsTimeseriesPoint;
use lib::model_mapper::Mapper;
use sqlx::FromRow;

#[derive(Mapper, FromRow)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(derive(ty = TransactionsTimeseriesPoint, into))]
pub struct StoredTransactionsTimeseriesPoint {
    pub bucket_start: DateTime<Utc>,

    pub tx_count: i64,

    pub gmv: f64,

    pub approval_rate: f32,

    pub decline_rate: f32,
}
