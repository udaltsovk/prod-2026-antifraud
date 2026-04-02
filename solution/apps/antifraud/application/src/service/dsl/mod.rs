use std::sync::Arc;

use domain::{
    fraud_rule::{FraudRule, dsl_expression::FraudRuleDslExpression},
    transaction::{CreateTransaction, decision::TransactionDecision},
    user::User,
};
use entrait::entrait;

pub use crate::service::dsl::error::{
    DslServiceError, DslServiceErrorExt, DslServiceErrorKind, DslServiceErrors,
    DslServiceResult,
};

mod error;

#[entrait(
    DslServiceImpl,
    delegate_by=DelegateDslService
)]
pub trait DslService {
    fn normalize_dsl(
        &self,
        expression: &FraudRuleDslExpression,
    ) -> DslServiceResult<FraudRuleDslExpression>;

    fn decide(
        &self,
        rules: &[FraudRule],
        input: Vec<(usize, (CreateTransaction, Arc<User>))>,
    ) -> Vec<(usize, TransactionDecision)>;
}
