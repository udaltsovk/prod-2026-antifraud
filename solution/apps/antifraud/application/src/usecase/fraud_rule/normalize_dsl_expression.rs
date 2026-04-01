use domain::fraud_rule::dsl_expression::FraudRuleDslExpression;
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    service::dsl::{DslService, DslServiceResult},
    usecase::fraud_rule::FraudRuleUseCaseResult,
};

#[entrait(pub NormalizeDslExpressionUsecase)]
#[instrument(skip(deps))]
fn normalize_dsl_expression<Deps>(
    deps: &Deps,
    expression: &FraudRuleDslExpression,
) -> FraudRuleUseCaseResult<DslServiceResult<FraudRuleDslExpression>>
where
    Deps: DslService,
{
    DslService::normalize_dsl(deps, expression).pipe(Ok)
}
