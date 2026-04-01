use domain::statistics::rules::RuleMatchesStats;
use lib::uuid::Uuid;
use model_mapper::Mapper;
use serde::Serialize;

pub mod filter;

#[derive(Mapper, Serialize, Debug)]
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
