use crate::{Expr, Literal, Operator, context::Context};

#[derive(Debug)]
pub enum ValidatorError<'src> {
    InvalidField {
        field: &'src str,
    },
    InvalidOperator {
        field: &'src str,
        op: Operator,
        value: Literal<'src>,
    },
}

#[derive(Debug, Clone)]
pub struct ValidationConfirmation(());

impl<'src> Expr<'src> {
    pub fn validate(
        &self,
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
        &self,
        ctx: &Context<'src>,
        errors: &mut Vec<ValidatorError<'src>>,
    ) {
        match self {
            Expr::Comparison {
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
                        validate_comparison(field, *op, *field_ty, value).err()
                    {
                        errors.push(err);
                    }
                },
            },

            Expr::Parens(expr) | Expr::Not(expr) => {
                expr.validate_into(ctx, errors);
            },

            Expr::And(lhs, rhs) | Expr::Or(lhs, rhs) => {
                lhs.validate_into(ctx, errors);
                rhs.validate_into(ctx, errors);
            },
        }
    }
}

const fn validate_comparison<'src>(
    field: &'src str,
    op: Operator,
    field_ty: Literal<'src>,
    value: &Literal<'src>,
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
                value: *value,
            }),
        },

        _ => Err(ValidatorError::InvalidOperator {
            field,
            op,
            value: *value,
        }),
    }
}
