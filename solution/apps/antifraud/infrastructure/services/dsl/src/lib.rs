#![feature(try_blocks)]

use std::{collections::HashMap, sync::Arc};

use application::service::dsl::{
    DslServiceErrorExt, DslServiceErrors, DslServiceImpl, DslServiceResult,
};
use domain::{
    fraud_rule::{
        FraudRule,
        dsl_expression::FraudRuleDslExpression,
        result::{
            FraudRuleResult, description::FraudRuleResultDescription,
            status::FraudRuleResultStatus,
        },
    },
    transaction::{CreateTransaction, decision::TransactionDecision},
    user::User,
};
use dsl::{Expression, ValidationConfirmation};
use entrait::entrait;
use lib::{domain::Id, instrument_all, tap::Pipe as _};
use rayon::iter::{
    IntoParallelIterator as _, IntoParallelRefIterator as _,
    ParallelIterator as _,
};

use crate::{context::DslServiceContext, utils::DslExpressionExt as _};

mod context;
mod utils;

#[derive(Clone)]
pub struct DslServiceImplementation {
    _phantom: (),
}

#[entrait]
#[instrument_all]
impl DslServiceImpl for DslServiceImplementation {
    fn normalize_dsl<App>(
        _app: &App,
        expression: &FraudRuleDslExpression,
    ) -> DslServiceResult<FraudRuleDslExpression> {
        let context = DslServiceContext::dummy();

        let ast = Expression::parse(expression.as_ref())
            .map_err(DslServiceErrorExt::into_dsl_service_errors)?;

        ast.validate(&context.0)
            .map_err(DslServiceErrorExt::into_dsl_service_errors)?;

        Ok(ast.into_domain())
    }

    fn decide<App>(
        _app: &App,
        rules: &[FraudRule],
        input: Vec<(usize, (CreateTransaction, Arc<User>))>,
    ) -> Vec<(usize, TransactionDecision)> {
        let dsl_expressions: HashMap<_, _> =
            rules.par_iter().map(FraudRuleExt::parse).collect();

        input
            .into_par_iter()
            .map(|(index, (transaction, user))| {
                let context = DslServiceContext::from((&transaction, &user));

                let rule_results: Vec<_> = rules
                    .par_iter()
                    .map(|rule| rule.decide(&dsl_expressions, &context))
                    .collect();

                (
                    index,
                    TransactionDecision {
                        transaction: transaction.commit(&rule_results),
                        rule_results,
                    },
                )
            })
            .collect()
    }
}

type FraudRuleId = Id<FraudRule>;

type RawFraudRuleResult<'src> =
    Result<(Expression<'src>, ValidationConfirmation), DslServiceErrors>;

trait FraudRuleExt {
    fn parse(&self) -> (FraudRuleId, RawFraudRuleResult<'_>);

    fn decide<'src>(
        &self,
        dsl_expressions: &HashMap<FraudRuleId, RawFraudRuleResult<'src>>,
        context: &DslServiceContext<'src>,
    ) -> FraudRuleResult;
}

impl FraudRuleExt for FraudRule {
    fn parse(&self) -> (FraudRuleId, RawFraudRuleResult<'_>) {
        let res = try {
            let context = DslServiceContext::dummy();

            let ast = Expression::parse(self.dsl_expression.as_ref())
                .map_err(DslServiceErrorExt::into_dsl_service_errors)?;

            let confirmation = ast
                .validate(&context.0)
                .map_err(DslServiceErrorExt::into_dsl_service_errors)?
                .clone();

            (ast, confirmation)
        };

        (self.id, res)
    }

    fn decide<'src>(
        &self,
        dsl_expressions: &HashMap<FraudRuleId, RawFraudRuleResult<'src>>,
        context: &DslServiceContext<'src>,
    ) -> FraudRuleResult {
        let expression_res = dsl_expressions.get(&self.id).expect(
            "we've created that map using every rule so result should exist",
        );

        self.apply(|_| match expression_res {
            Ok((ast, confirmation)) => {
                let status: FraudRuleResultStatus =
                    ast.evaluate(&context.0, confirmation).into();

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
                    .pipe(FraudRuleResultDescription),
            ),
        })
    }
}

#[instrument_all(level = "trace")]
impl DslServiceImplementation {
    pub fn new() -> Self {
        Self {
            _phantom: (),
        }
    }
}

impl Default for DslServiceImplementation {
    fn default() -> Self {
        Self::new()
    }
}
