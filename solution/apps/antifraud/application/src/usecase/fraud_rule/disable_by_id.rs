use domain::fraud_rule::{FraudRule, status::FraudRuleStatus};
use entrait::entrait;
use lib::{
    domain::Id,
    tap::{Pipe as _, Tap as _},
};
use tracing::instrument;

use crate::{
    repository::fraud_rule::FraudRuleRepository,
    usecase::fraud_rule::{
        FraudRuleUseCaseResult, get_by_id::GetFraudRuleByIdUsecase,
    },
};

#[entrait(pub DisableFraudRuleByIdUsecase)]
#[instrument(skip(deps))]
async fn disable_fraud_rule_by_id<Deps>(
    deps: &Deps,
    fraud_rule_id: Id<FraudRule>,
) -> FraudRuleUseCaseResult<FraudRule>
where
    Deps: GetFraudRuleByIdUsecase + FraudRuleRepository,
{
    let fraud_rule =
        GetFraudRuleByIdUsecase::get_fraud_rule_by_id(deps, fraud_rule_id)
            .await?;

    if fraud_rule.status == FraudRuleStatus::Disabled {
        return Ok(fraud_rule);
    }

    let updated_fraud_rule = fraud_rule.tap_mut(|fraud_rule| {
        fraud_rule.status = FraudRuleStatus::Disabled;
    });

    FraudRuleRepository::update_fraud_rule(deps, updated_fraud_rule)
        .await?
        .pipe(Ok)
}
