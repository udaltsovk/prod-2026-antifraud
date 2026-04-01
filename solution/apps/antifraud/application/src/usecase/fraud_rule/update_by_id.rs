use domain::fraud_rule::{FraudRule, FraudRuleUpdate};
use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::fraud_rule::FraudRuleRepository,
    usecase::fraud_rule::{
        FraudRuleUseCaseError, FraudRuleUseCaseResult,
        find_by_name::FindFraudRuleByNameUsecase,
        get_by_id::GetFraudRuleByIdUsecase,
    },
};

#[entrait(pub UpdateFraudRuleByIdUsecase)]
#[instrument(skip(deps))]
async fn update_fraud_rule_by_id<Deps>(
    deps: &Deps,
    fraud_rule_id: Id<FraudRule>,
    update: FraudRuleUpdate,
) -> FraudRuleUseCaseResult<FraudRule>
where
    Deps: GetFraudRuleByIdUsecase
        + FindFraudRuleByNameUsecase
        + FraudRuleRepository,
{
    let fraud_rule =
        GetFraudRuleByIdUsecase::get_fraud_rule_by_id(deps, fraud_rule_id)
            .await?;

    if update.eq(&fraud_rule) {
        return Ok(fraud_rule);
    }

    if let Some(rule) =
        FindFraudRuleByNameUsecase::find_fraud_rule_by_name(deps, &update.name)
            .await?
        && rule.id != fraud_rule.id
        && update.name == rule.name
    {
        return FraudRuleUseCaseError::NameAlreadyUsed(update.name).pipe(Err);
    }

    let updated_fraud_rule = update.apply_to(fraud_rule);

    FraudRuleRepository::update_fraud_rule(deps, updated_fraud_rule)
        .await?
        .pipe(Ok)
}
