use domain::fraud_rule::dsl_expression::FraudRuleDslExpression;

pub use crate::service::dsl::error::{
    DslServiceError, DslServiceErrorKind, DslServiceResult,
};

mod error;

pub trait DslService {
    fn normalize(
        &self,
        expression: FraudRuleDslExpression,
    ) -> DslServiceResult<FraudRuleDslExpression>;

    fn evaluate(
        &self,
        expression: FraudRuleDslExpression,
    ) -> DslServiceResult<bool>;
}
