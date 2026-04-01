use domain::statistics::rules::RuleMatchesStats;
use lib::{infrastructure::persistence::entity::DomainTypeFromDb, uuid::Uuid};
use model_mapper::Mapper;
use sqlx::FromRow;

#[derive(Mapper, FromRow, Debug)]
#[mapper(derive(ty = RuleMatchesStats, into))]
pub struct StoredRuleMatchesStats {
    pub rule_id: Uuid,

    #[mapper(
        when(ty = RuleMatchesStats, into_with = DomainTypeFromDb::into_domain),
    )]
    pub rule_name: String,

    pub matches: i64,

    pub unique_users: i64,

    pub unique_merchants: i64,

    pub share_of_declines: f32,
}
