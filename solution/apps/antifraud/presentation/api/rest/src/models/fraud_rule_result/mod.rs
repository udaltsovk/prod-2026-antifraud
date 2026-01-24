use domain::fraud_rule::result::FraudRuleResult;
use lib::{model_mapper::Mapper, uuid::Uuid};
use serde::Serialize;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = FraudRuleResult, from)]
#[serde(rename_all = "UPPERCASE")]
pub struct JsonFraudRuleResult {
    pub rule_id: Uuid,

    pub rule_name: String,

    pub priority: i64,

    #[mapper(rename = status)]
    pub matched: bool,

    pub description: String,
}
