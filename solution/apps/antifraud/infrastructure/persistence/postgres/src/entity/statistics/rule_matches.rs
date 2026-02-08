use domain::statistics::rules::RuleMatchesStats;
use lib::{
    infrastructure::persistence::entity::DomainTypeFromDb,
    model_mapper::Mapper, uuid::Uuid,
};
use sqlx::FromRow;

#[derive(Mapper, FromRow)]
#[cfg_attr(debug_assertions, derive(Debug))]
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
