use domain::transaction::decision::TransactionDecision;
use entrait::entrait;
use lib::tap::Pipe as _;
use tracing::instrument;

use crate::{
    repository::{
        fraud_rule_result::FraudRuleResultRepository,
        transaction::TransactionRepository,
    },
    usecase::transaction::TransactionUseCaseResult,
};

#[entrait(pub SaveTransactionDecisionUsecase)]
#[instrument(skip(deps))]
async fn save_transaction_decision<Deps>(
    deps: &Deps,
    TransactionDecision {
        transaction,
        rule_results,
    }: TransactionDecision,
) -> TransactionUseCaseResult<TransactionDecision>
where
    Deps: TransactionRepository + FraudRuleResultRepository,
{
    let transaction =
        TransactionRepository::save_transaction(deps, transaction).await?;

    let rule_results =
        FraudRuleResultRepository::batch_create_fraud_rule_results(
            deps,
            (transaction.id, rule_results),
        )
        .await?;

    TransactionDecision {
        transaction,
        rule_results,
    }
    .pipe(Ok)
}
