use domain::transaction::decision::TransactionDecision;
use lib::model_mapper::Mapper;
use serde::Serialize;

use crate::dto::{
    fraud_rule_result::FraudRuleResultDto, transaction::TransactionDto,
};

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = TransactionDecision, from)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDecisionDto {
    pub transaction: TransactionDto,

    #[mapper(iter)]
    pub rule_results: Vec<FraudRuleResultDto>,
}
