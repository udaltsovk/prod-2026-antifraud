use chrono::{DateTime, Utc};
use domain::statistics::users::UserRiskProfile;
use lib::uuid::Uuid;
use model_mapper::Mapper;
use sqlx::FromRow;

#[derive(Mapper, FromRow, Debug)]
#[mapper(derive(ty = UserRiskProfile, into))]
pub struct StoredUserRiskProfile {
    pub user_id: Uuid,

    pub tx_count_24h: i64,

    pub gmv_24h: f64,

    pub distinct_devices_24h: i64,

    pub distinct_ips_24h: i64,

    pub distinct_cities_24h: i64,

    pub decline_rate_30d: f32,

    pub last_seen_at: Option<DateTime<Utc>>,
}
