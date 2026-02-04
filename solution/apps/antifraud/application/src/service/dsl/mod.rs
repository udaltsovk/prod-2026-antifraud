use std::sync::Arc;

use domain::{
    fraud_rule::{FraudRule, dsl_expression::FraudRuleDslExpression},
    transaction::{CreateTransaction, decision::TransactionDecision},
    user::User,
};

pub use crate::service::dsl::error::{
    DslServiceError, DslServiceErrorExt, DslServiceErrorKind, DslServiceErrors,
    DslServiceResult,
};

mod error;

pub trait DslService {
    fn normalize(
        &self,
        expression: FraudRuleDslExpression,
    ) -> DslServiceResult<FraudRuleDslExpression>;

    fn decide(
        &self,
        rules: &[FraudRule],
        input: Vec<(usize, (CreateTransaction, Arc<User>))>,
    ) -> Vec<(usize, TransactionDecision)>;
}
