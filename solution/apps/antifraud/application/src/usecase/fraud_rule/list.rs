use domain::fraud_rule::{FraudRule, status::FraudRuleStatus};
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    repository::fraud_rule::FraudRuleRepository,
    usecase::fraud_rule::FraudRuleUseCaseResult,
};

#[entrait(pub ListFraudRulesUsecase)]
#[instrument(skip(deps))]
async fn list_fraud_rules<Deps>(
    deps: &Deps,
    status: Option<FraudRuleStatus>,
) -> FraudRuleUseCaseResult<Vec<FraudRule>>
where
    Deps: FraudRuleRepository,
{
    FraudRuleRepository::list_fraud_rules(deps, status)
        .await?
        .pipe(Ok)
}
