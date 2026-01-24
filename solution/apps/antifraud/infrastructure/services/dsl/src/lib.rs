use application::service::dsl::{
    DslService, DslServiceError, DslServiceErrorKind, DslServiceResult,
};
use domain::fraud_rule::dsl_expression::FraudRuleDslExpression;
use lib::instrument_all;

pub struct DslServiceImpl {
    _phantom: (),
}

#[instrument_all]
impl DslService for DslServiceImpl {
    fn normalize(
        &self,
        _expression: FraudRuleDslExpression,
    ) -> DslServiceResult<FraudRuleDslExpression> {
        Err(Self::unsupported_tier_error())
    }

    fn evaluate(
        &self,
        _expression: FraudRuleDslExpression,
    ) -> DslServiceResult<bool> {
        Err(Self::unsupported_tier_error())
    }
}

#[instrument_all(level = "trace")]
impl DslServiceImpl {
    pub fn new() -> Self {
        Self {
            _phantom: (),
        }
    }

    pub fn unsupported_tier_error() -> Vec<DslServiceError> {
        let error = DslServiceError {
            kind: DslServiceErrorKind::ParseError,
            message: "Неподдерживаемый уровень".into(),
            position: None,
            near: None,
        };
        vec![error]
    }
}

impl Default for DslServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
