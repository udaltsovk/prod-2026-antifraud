use domain::fraud_rule::result::FraudRuleResult;
use lib::{infrastructure::persistence::entity::DomainTypeFromDb, uuid::Uuid};
use model_mapper::Mapper;
use sqlx::FromRow;

#[derive(Mapper, FromRow, Debug)]
#[mapper(derive(ty = FraudRuleResult, into))]
pub struct StoredFraudRuleResult {
    #[mapper(
        when(ty = FraudRuleResult, skip)
    )]
    pub transaction_id: Uuid,

    pub rule_id: Uuid,

    #[mapper(
        when(ty = FraudRuleResult, into_with = DomainTypeFromDb::into_domain),
    )]
    pub rule_name: String,

    #[mapper(
        when(ty = FraudRuleResult, into_with = DomainTypeFromDb::into_domain),
    )]
    pub priority: i64,

    #[mapper(
        when(ty = FraudRuleResult, rename = status)
    )]
    pub matched: bool,

    #[mapper(
        when(ty = FraudRuleResult, into_with = DomainTypeFromDb::into_domain),
    )]
    pub description: String,
}
