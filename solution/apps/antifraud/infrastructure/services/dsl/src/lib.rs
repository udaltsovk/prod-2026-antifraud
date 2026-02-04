use std::{collections::HashMap, sync::Arc};

use application::service::dsl::{
    DslService, DslServiceErrorExt, DslServiceErrors, DslServiceResult,
};
use domain::{
    fraud_rule::{
        FraudRule,
        dsl_expression::FraudRuleDslExpression,
        result::{
            description::FraudRuleResultDescription,
            status::FraudRuleResultStatus,
        },
    },
    transaction::{CreateTransaction, decision::TransactionDecision},
    user::User,
};
use dsl::Expression;
use lib::{instrument_all, tap::Pipe as _};

use crate::{context::DslServiceContext, utils::DslExpressionExt as _};

mod context;
mod utils;

#[derive(Clone)]
pub struct DslServiceImpl {
    _phantom: (),
}

#[instrument_all]
impl DslService for DslServiceImpl {
    fn normalize(
        &self,
        expression: FraudRuleDslExpression,
    ) -> DslServiceResult<FraudRuleDslExpression> {
        let context = DslServiceContext::dummy();

        let ast = Expression::parse(expression.as_ref())
            .map_err(DslServiceErrorExt::into_dsl_service_errors)?;

        ast.validate(&context.0)
            .map_err(DslServiceErrorExt::into_dsl_service_errors)?;

        Ok(ast.into_domain())
    }

    fn decide(
        &self,
        rules: &[FraudRule],
        input: Vec<(usize, (CreateTransaction, Arc<User>))>,
    ) -> Vec<(usize, TransactionDecision)> {
        let dsl_expressions: HashMap<_, _> = rules
            .iter()
            .map(|rule| {
                let res: Result<_, DslServiceErrors> = (|| {
                    let context = DslServiceContext::dummy();

                    let ast = Expression::parse(rule.dsl_expression.as_ref())
                        .map_err(
                        DslServiceErrorExt::into_dsl_service_errors,
                    )?;

                    let confirmation = ast
                        .validate(&context.0)
                        .map_err(DslServiceErrorExt::into_dsl_service_errors)?
                        .clone();

                    Ok((ast, confirmation))
                })();

                (rule.id, res)
            })
            .collect();

        input
            .into_iter()
            .map(|(index, (transaction, user))| {
                let context = DslServiceContext::from((&transaction, &user));
                let rule_results: Vec<_> = rules
                    .iter()
                    .map(|rule| {
                        let expression_res = dsl_expressions
                            .get(&rule.id)
                            .expect(
                                "we've created that map using every rule so result should exist"
                            );

                        rule.apply(|_| {
                            match expression_res {
                                Ok((ast, confirmation)) => {
                                    let status: FraudRuleResultStatus = ast
                                        .evaluate(&context.0, confirmation)
                                        .into();

                                    let matched = if status.eq(&FraudRuleResultStatus::Unmatched) {
                                        "не "
                                    } else {
                                        ""
                                    };

                                    let description = format!("{ast}, правило {matched}сработало")
                                        .pipe(FraudRuleResultDescription);

                                    (status, description)
                                },
                                Err(_err) => (
                                    FraudRuleResultStatus::Unmatched,
                                    "Fraud rule DSL expression is not valid"
                                        .to_string()
                                        .pipe(FraudRuleResultDescription)
                                )
                            }

                        })
                    })
                    .collect();

                    (
                        index,
                        TransactionDecision {
                            transaction: transaction.commit(&rule_results),
                            rule_results,
                        }
                    )
                })
            .collect()
    }
}

#[instrument_all(level = "trace")]
impl DslServiceImpl {
    pub fn new() -> Self {
        Self {
            _phantom: (),
        }
    }
}

impl Default for DslServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
