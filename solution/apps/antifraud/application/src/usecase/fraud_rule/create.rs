use domain::fraud_rule::{CreateFraudRule, FraudRule};
use entrait::entrait;
use lib::{domain::Id, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::fraud_rule::FraudRuleRepository,
    usecase::fraud_rule::{
        FraudRuleUseCaseError, FraudRuleUseCaseResult,
        find_by_name::FindFraudRuleByNameUsecase,
    },
};

#[entrait(pub CreateFraudRuleUsecase)]
#[instrument(skip(deps))]
async fn create_fraud_rule<Deps>(
    deps: &Deps,
    source: CreateFraudRule,
) -> FraudRuleUseCaseResult<FraudRule>
where
    Deps: FindFraudRuleByNameUsecase + FraudRuleRepository,
{
    if FindFraudRuleByNameUsecase::find_fraud_rule_by_name(deps, &source.name)
        .await?
        .is_some()
    {
        return FraudRuleUseCaseError::NameAlreadyUsed(source.name).pipe(Err);
    }

    FraudRuleRepository::create_fraud_rule(deps, (Id::generate(), source))
        .await?
        .pipe(Ok)
}
