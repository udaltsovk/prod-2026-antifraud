use application::service::dsl::{
    DslService, DslServiceError, DslServiceErrorKind, DslServiceResult,
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
use lib::{instrument_all, tap::Pipe as _};

pub struct DslServiceImpl {
    _phantom: (),
}

#[instrument_all]
impl DslService for DslServiceImpl {
    fn normalize(
        &self,
        _expression: FraudRuleDslExpression,
    ) -> DslServiceResult<FraudRuleDslExpression> {
        let error = DslServiceError {
            kind: DslServiceErrorKind::ParseError,
            message: Self::unsupported_tier_msg(),
            position: None,
            near: None,
        };
        Err(vec![error])
    }

    fn decide(
        &self,
        rules: &[FraudRule],
        transaction: CreateTransaction,
        _user: &User,
    ) -> TransactionDecision {
        let rule_results: Vec<_> = rules
            .iter()
            .map(|rule| {
                rule.apply(|_| {
                    (
                        FraudRuleResultStatus::Unmatched,
                        Self::unsupported_tier_msg()
                            .pipe(FraudRuleResultDescription),
                    )
                })
            })
            .collect();

        TransactionDecision {
            transaction: transaction.commit(&rule_results),
            rule_results,
        }
    }
}

#[instrument_all(level = "trace")]
impl DslServiceImpl {
    pub fn new() -> Self {
        Self {
            _phantom: (),
        }
    }

    pub fn unsupported_tier_msg() -> String {
        "Неподдерживаемый уровень".into()
    }
}

impl Default for DslServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
