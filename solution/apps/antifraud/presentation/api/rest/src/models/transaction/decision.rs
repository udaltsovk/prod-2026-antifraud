use domain::transaction::decision::TransactionDecision;
use lib::model_mapper::Mapper;
use serde::Serialize;

use crate::models::{
    fraud_rule_result::JsonFraudRuleResult, transaction::JsonTransaction,
};

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = TransactionDecision, from)]
#[serde(rename_all = "camelCase")]
pub struct JsonTransactionDecision {
    pub transaction: JsonTransaction,

    #[mapper(iter)]
    pub rule_results: Vec<JsonFraudRuleResult>,
}
