#![expect(
    clippy::only_used_in_recursion,
    clippy::unreachable,
    reason = "that's how our silly validation check works"
)]

use crate::{
    Context, Expression, Literal, Operator, validator::ValidationConfirmation,
};

impl<'src> Expression<'src> {
    const ERROR_MARGIN: f64 = 0.01;

    #[must_use]
    pub fn evaluate(
        &self,
        ctx: &Context<'src>,
        confirmation: &'src ValidationConfirmation,
    ) -> bool {
        match self {
            Expression::Comparison {
                field,
                op,
                value,
            } => {
                let field_value =
                    ctx.get_field(field).expect("Field validated already");

                match (field_value, value) {
                    (
                        Literal::Number(Some(lhs)),
                        Literal::Number(Some(rhs)),
                    ) => match op {
                        Operator::GreaterThan => lhs > rhs,
                        Operator::GreaterThanOrEqual => lhs >= rhs,
                        Operator::LessThan => lhs < rhs,
                        Operator::LessThanOrEqual => lhs <= rhs,
                        Operator::Equal => {
                            (lhs - rhs).abs() < Self::ERROR_MARGIN
                        },
                        Operator::NotEqual => {
                            (lhs - rhs).abs() >= Self::ERROR_MARGIN
                        },
                    },
                    (Literal::Number(None), Literal::Number(Some(_))) => false,
                    (Literal::String(lhs), Literal::String(rhs)) => match op {
                        Operator::Equal => lhs == rhs,
                        Operator::NotEqual => lhs != rhs,

                        _ => unreachable!(
                            "Other ops were rejected at validation"
                        ),
                    },
                    _ => unreachable!(
                        "Mixed types should be impossible due to validation"
                    ),
                }
            },

            Expression::Parens(inner) => inner.evaluate(ctx, confirmation),
            Expression::Not(inner) => !inner.evaluate(ctx, confirmation),
            Expression::And(lhs, rhs) => {
                lhs.evaluate(ctx, confirmation)
                    && rhs.evaluate(ctx, confirmation)
            },
            Expression::Or(lhs, rhs) => {
                lhs.evaluate(ctx, confirmation)
                    || rhs.evaluate(ctx, confirmation)
            },
        }
    }
}
