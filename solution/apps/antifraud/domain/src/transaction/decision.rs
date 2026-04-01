use crate::{fraud_rule::result::FraudRuleResult, transaction::Transaction};

#[derive(Debug)]
pub struct TransactionDecision {
    pub transaction: Transaction,
    pub rule_results: Vec<FraudRuleResult>,
}
