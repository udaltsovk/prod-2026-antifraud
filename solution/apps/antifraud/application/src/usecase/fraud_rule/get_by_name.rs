use domain::fraud_rule::{FraudRule, name::FraudRuleName};
use entrait::entrait;
use tracing::instrument;

use crate::usecase::fraud_rule::{
    FraudRuleUseCaseError, FraudRuleUseCaseResult,
    find_by_name::FindFraudRuleByNameUsecase,
};

#[entrait(pub GetFraudRuleByNameUsecase)]
#[instrument(skip(app))]
async fn get_fraud_rule_by_name<App>(
    app: &App,
    fraud_rule_name: FraudRuleName,
) -> FraudRuleUseCaseResult<FraudRule>
where
    App: FindFraudRuleByNameUsecase,
{
    app.find_fraud_rule_by_name(&fraud_rule_name)
        .await?
        .ok_or(FraudRuleUseCaseError::NotFoundByName(fraud_rule_name))
}
