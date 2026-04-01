use domain::fraud_rule::FraudRule;
use entrait::entrait;
use lib::domain::Id;
use tracing::instrument;

use crate::usecase::fraud_rule::{
    FraudRuleUseCaseError, FraudRuleUseCaseResult,
    find_by_id::FindFraudRuleByIdUsecase,
};

#[entrait(pub GetFraudRuleByIdUsecase)]
#[instrument(skip(deps))]
async fn get_fraud_rule_by_id<Deps>(
    deps: &Deps,
    fraud_rule_id: Id<FraudRule>,
) -> FraudRuleUseCaseResult<FraudRule>
where
    Deps: FindFraudRuleByIdUsecase,
{
    FindFraudRuleByIdUsecase::find_fraud_rule_by_id(deps, fraud_rule_id)
        .await?
        .ok_or(FraudRuleUseCaseError::NotFoundById(fraud_rule_id))
}
