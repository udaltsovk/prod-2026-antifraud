use domain::fraud_rule::result::FraudRuleResult;
use lib::uuid::Uuid;
use model_mapper::Mapper;
use serde::Serialize;

#[derive(Mapper, Serialize, Debug)]
#[mapper(ty = FraudRuleResult, from)]
#[serde(rename_all = "camelCase")]
pub struct FraudRuleResultDto {
    pub rule_id: Uuid,

    pub rule_name: String,

    pub priority: i64,

    #[mapper(rename = status)]
    pub matched: bool,

    pub description: String,
}
