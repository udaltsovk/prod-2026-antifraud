use domain::statistics::merchants::MerchantRiskStats;
use lib::model_mapper::Mapper;
use serde::Serialize;

pub mod filter;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = MerchantRiskStats, from)]
#[serde(rename_all = "camelCase")]
pub struct MerchantRiskStatsDto {
    pub merchant_id: String,

    #[mapper(opt)]
    pub merchant_category_code: Option<String>,

    pub tx_count: i64,

    pub gmv: f64,

    pub decline_rate: f32,
}
