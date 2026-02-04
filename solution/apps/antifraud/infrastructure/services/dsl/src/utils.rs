use domain::fraud_rule::dsl_expression::FraudRuleDslExpression;
use dsl::Expression;

pub trait DslExpressionExt {
    fn into_domain(self) -> FraudRuleDslExpression;
}

impl DslExpressionExt for Expression<'_> {
    fn into_domain(self) -> FraudRuleDslExpression {
        FraudRuleDslExpression::try_from(self.to_string()).expect(
            "any valid dsl expression should pass the domain validation",
        )
    }
}
