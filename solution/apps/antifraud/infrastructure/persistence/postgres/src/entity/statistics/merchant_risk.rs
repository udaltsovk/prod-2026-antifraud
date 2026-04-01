use domain::statistics::merchants::MerchantRiskStats;
use lib::infrastructure::persistence::entity::DomainTypeFromDb;
use model_mapper::Mapper;
use sqlx::{FromRow, Type};

#[derive(Mapper, Type, FromRow, Debug)]
#[mapper(derive(ty = MerchantRiskStats, into))]
#[sqlx(type_name = "merchant_risk_stats")]
pub struct StoredMerchantRiskStats {
    #[mapper(
        when(ty = MerchantRiskStats, into_with = DomainTypeFromDb::into_domain),
    )]
    pub merchant_id: String,

    #[mapper(
        when(ty = MerchantRiskStats, opt(
            into_with = DomainTypeFromDb::into_domain
        )),
    )]
    pub merchant_category_code: Option<String>,

    pub tx_count: i64,

    pub gmv: f64,

    pub decline_rate: f32,
}
