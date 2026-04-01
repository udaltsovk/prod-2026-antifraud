use domain::fraud_rule::{FraudRule, name::FraudRuleName};
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    repository::fraud_rule::FraudRuleRepository,
    usecase::fraud_rule::FraudRuleUseCaseResult,
};

#[entrait(pub FindFraudRuleByNameUsecase)]
#[instrument(skip(app))]
async fn find_fraud_rule_by_name<App>(
    app: &App,
    fraud_rule_name: &FraudRuleName,
) -> FraudRuleUseCaseResult<Option<FraudRule>>
where
    App: FraudRuleRepository,
{
    app.find_fraud_rule_by_name(fraud_rule_name).await?.pipe(Ok)
}
