use lib::domain::Id;

use crate::fraud_rule::{FraudRule, name::FraudRuleName};

pub mod filter;

#[derive(Debug)]
pub struct RuleMatchesStats {
    pub rule_id: Id<FraudRule>,
    pub rule_name: FraudRuleName,
    pub matches: i64,
    pub unique_users: i64,
    pub unique_merchants: i64,
    pub share_of_declines: f32,
}
