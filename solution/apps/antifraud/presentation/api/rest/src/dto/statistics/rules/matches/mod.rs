use domain::statistics::rules::RuleMatchesStats;
use lib::{model_mapper::Mapper, uuid::Uuid};
use serde::Serialize;

pub mod filter;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = RuleMatchesStats, from)]
#[serde(rename_all = "camelCase")]
pub struct RuleMatchesStatsDto {
    pub rule_id: Uuid,

    pub rule_name: String,

    pub matches: i64,

    pub unique_users: i64,

    pub unique_merchants: i64,

    pub share_of_declines: f32,
}
