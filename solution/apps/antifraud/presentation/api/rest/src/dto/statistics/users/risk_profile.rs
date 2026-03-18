use chrono::{DateTime, Utc};
use domain::statistics::users::UserRiskProfile;
use lib::{model_mapper::Mapper, uuid::Uuid};
use serde::Serialize;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = UserRiskProfile, from)]
#[serde(rename_all = "camelCase")]
pub struct UserRiskProfileDto {
    pub user_id: Uuid,

    pub tx_count_24h: i64,

    #[serde(rename = "gmv_24h")]
    pub gmv_24h: f64,

    #[serde(rename = "distinctDevices_24h")]
    pub distinct_devices_24h: i64,

    #[serde(rename = "distinctIps_24h")]
    pub distinct_ips_24h: i64,

    #[serde(rename = "distinctCities_24h")]
    pub distinct_cities_24h: i64,

    #[serde(rename = "declineRate_30d")]
    pub decline_rate_30d: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<DateTime<Utc>>,
}
