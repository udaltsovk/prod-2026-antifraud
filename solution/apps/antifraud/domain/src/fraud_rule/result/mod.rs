use lib::domain::Id;

use crate::fraud_rule::{
    FraudRule,
    name::FraudRuleName,
    priority::FraudRulePriority,
    result::{
        description::FraudRuleResultDescription, status::FraudRuleResultStatus,
    },
};

pub mod description;
pub mod status;

#[derive(Debug)]
pub struct FraudRuleResult {
    pub rule_id: Id<FraudRule>,
    pub rule_name: FraudRuleName,
    pub priority: FraudRulePriority,
    pub status: FraudRuleResultStatus,
    pub description: FraudRuleResultDescription,
}
