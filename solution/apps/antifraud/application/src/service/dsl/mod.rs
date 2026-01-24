use domain::{
    fraud_rule::{FraudRule, dsl_expression::FraudRuleDslExpression},
    transaction::{CreateTransaction, decision::TransactionDecision},
    user::User,
};

pub use crate::service::dsl::error::{
    DslServiceError, DslServiceErrorKind, DslServiceResult,
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
        transaction: CreateTransaction,
        user: &User,
    ) -> TransactionDecision;
}
