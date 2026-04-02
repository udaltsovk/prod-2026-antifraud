use application::repository::fraud_rule_result::FraudRuleResultRepositoryImpl;
use domain::{fraud_rule::result::FraudRuleResult, transaction::Transaction};
use entrait::entrait;
use lib::{
    anyhow::Result,
    application::di::Has,
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::{HasPoolExt as _, sqlx::SqlxPool},
    instrument_all,
    tap::Pipe as _,
    uuid::Uuid,
};
use sqlx::{Acquire as _, Executor, Postgres, query_file_as};

use crate::{
    entity::fraud_rule_result::StoredFraudRuleResult,
    repository::PostgresRepositoryImpl,
};

#[entrait]
#[async_trait]
#[instrument_all]
impl FraudRuleResultRepositoryImpl for PostgresRepositoryImpl {
    async fn batch_create_fraud_rule_results<Deps>(
        deps: &Deps,
        (transaction_id, sources): (Id<Transaction>, Vec<FraudRuleResult>),
    ) -> Result<Vec<FraudRuleResult>>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let transaction_id = transaction_id.value;

        let mut fraud_rule_results = Vec::new();

        let mut connection = deps.get_connection().await?;
        let mut transaction = connection.begin().await?;

        for source in sources {
            save_fraud_rule_result(&mut *transaction, (transaction_id, source))
                .await
                .map(|transaction| fraud_rule_results.push(transaction))?;
        }

        transaction.commit().await?;

        Ok(fraud_rule_results)
    }

    async fn find_all_fraud_rule_results_by_transaction_id<Deps>(
        deps: &Deps,
        transaction_id: Id<Transaction>,
    ) -> Result<Vec<FraudRuleResult>>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let transaction_id = transaction_id.value;

        let mut connection = deps.get_connection().await?;

        query_file_as!(
            StoredFraudRuleResult,
            "sql/fraud_rule_result/find_all_by_transaction_id.sql",
            transaction_id
        )
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(FraudRuleResult::from)
        .collect::<Vec<_>>()
        .pipe(Ok)
    }
}

async fn save_fraud_rule_result<'c, E>(
    executor: E,
    (transaction_id, source): (Uuid, FraudRuleResult),
) -> Result<FraudRuleResult>
where
    E: Executor<'c, Database = Postgres>,
{
    let rule_id = source.rule_id.value;
    let rule_name = source.rule_name.into_inner();
    let priority = source.priority.into_inner();
    let matched = source.status.to_bool();
    let description = source.description.into_inner();

    query_file_as!(
        StoredFraudRuleResult,
        "sql/fraud_rule_result/create.sql",
        transaction_id,
        rule_id,
        rule_name,
        priority,
        matched,
        description
    )
    .fetch_one(executor)
    .await
    .map_err(Into::into)
    .map(FraudRuleResult::from)
}
