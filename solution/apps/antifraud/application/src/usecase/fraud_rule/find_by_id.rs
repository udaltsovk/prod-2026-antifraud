use domain::fraud_rule::FraudRule;
use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::fraud_rule::FraudRuleRepository,
    usecase::fraud_rule::FraudRuleUseCaseResult,
};

#[entrait(pub FindFraudRuleByIdUsecase)]
#[instrument(skip(deps))]
async fn find_fraud_rule_by_id<Deps>(
    deps: &Deps,
    fraud_rule_id: Id<FraudRule>,
) -> FraudRuleUseCaseResult<Option<FraudRule>>
where
    Deps: FraudRuleRepository,
{
    FraudRuleRepository::find_fraud_rule_by_id(deps, fraud_rule_id)
        .await?
        .pipe(Ok)
}
