use chrono::{DateTime, Utc};
use lib::domain::Id;

use crate::user::User;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserRiskProfile {
    pub user_id: Id<User>,
    pub tx_count_24h: i64,
    pub gmv_24h: f64,
    pub distinct_devices_24h: i64,
    pub distinct_ips_24h: i64,
    pub distinct_cities_24h: i64,
    pub decline_rate_30d: f32,
    pub last_seen_at: Option<DateTime<Utc>>,
}
