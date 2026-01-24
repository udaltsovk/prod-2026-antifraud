use crate::{fraud_rule::result::FraudRuleResult, transaction::Transaction};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionDecision {
    pub transaction: Transaction,
    pub rule_results: Vec<FraudRuleResult>,
}
