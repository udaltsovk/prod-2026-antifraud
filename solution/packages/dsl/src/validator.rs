use crate::{Expression, Literal, Operator, context::Context};

#[derive(Debug)]
pub enum ValidatorError<'src> {
    InvalidField {
        field: &'src str,
    },
    InvalidOperator {
        field: &'src str,
        op: Operator,
        value: &'src Literal,
    },
}

#[derive(Debug, Clone)]
pub struct ValidationConfirmation(());

impl<'src> Expression<'src> {
    pub fn validate(
        &'src self,
        ctx: &Context<'src>,
    ) -> Result<&'src ValidationConfirmation, Vec<ValidatorError<'src>>> {
        let mut errors = Vec::new();
        self.validate_into(ctx, &mut errors);
        errors
            .is_empty()
            .then_some(&ValidationConfirmation(()))
            .ok_or(errors)
    }

    fn validate_into(
        &'src self,
        ctx: &Context<'src>,
        errors: &mut Vec<ValidatorError<'src>>,
    ) {
        match self {
            Expression::Comparison {
                field,
                op,
                value,
            } => match ctx.get_field(field) {
                None => {
                    errors.push(ValidatorError::InvalidField {
                        field,
                    });
                },
                Some(field_ty) => {
                    if let Some(err) =
                        validate_comparison(field, *op, field_ty, value).err()
                    {
                        errors.push(err);
                    }
                },
            },

            Expression::Parens(expr) | Expression::Not(expr) => {
                expr.validate_into(ctx, errors);
            },

            Expression::And(lhs, rhs) | Expression::Or(lhs, rhs) => {
                lhs.validate_into(ctx, errors);
                rhs.validate_into(ctx, errors);
            },
        }
    }
}

const fn validate_comparison<'src>(
    field: &'src str,
    op: Operator,
    field_ty: &Literal,
    value: &'src Literal,
) -> Result<(), ValidatorError<'src>> {
    use Literal as Lit;
    use Operator as Op;

    match (field_ty, value) {
        (Lit::Number(_), Lit::Number(_)) => Ok(()),

        (Lit::String(_), Lit::String(_)) => match op {
            Op::Equal | Op::NotEqual => Ok(()),
            _ => Err(ValidatorError::InvalidOperator {
                field,
                op,
                value,
            }),
        },

        _ => Err(ValidatorError::InvalidOperator {
            field,
            op,
            value,
        }),
    }
}
