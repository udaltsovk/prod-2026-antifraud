use application::repository::fraud_rule_result::FraudRuleResultRepository;
use domain::{fraud_rule::result::FraudRuleResult, transaction::Transaction};
use lib::{
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::postgres::error::PostgresAdapterError,
    instrument_all,
    tap::Pipe as _,
    uuid::Uuid,
};
use sqlx::{Acquire as _, Executor, Postgres, query_file_as};

use crate::{
    entity::fraud_rule_result::StoredFraudRuleResult,
    repository::PostgresRepositoryImpl,
};

#[async_trait]
#[instrument_all]
impl FraudRuleResultRepository for PostgresRepositoryImpl<FraudRuleResult> {
    type AdapterError = PostgresAdapterError;

    async fn batch_create(
        &self,
        (transaction_id, sources): (Id<Transaction>, Vec<FraudRuleResult>),
    ) -> Result<Vec<FraudRuleResult>, Self::AdapterError> {
        let transaction_id = transaction_id.value;

        let mut fraud_rule_results = Vec::new();

        let mut connection = self.pool.get().await?;
        let mut transaction = connection.begin().await?;

        for source in sources {
            Self::save_fraud_rule_result(
                &mut *transaction,
                (transaction_id, source),
            )
            .await
            .map(|transaction| fraud_rule_results.push(transaction))?;
        }

        transaction.commit().await?;

        Ok(fraud_rule_results)
    }

    async fn find_all_by_transaction_id(
        &self,
        transaction_id: Id<Transaction>,
    ) -> Result<Vec<FraudRuleResult>, Self::AdapterError> {
        let transaction_id = transaction_id.value;

        let mut connection = self.pool.get().await?;

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

#[instrument_all]
impl PostgresRepositoryImpl<FraudRuleResult> {
    async fn save_fraud_rule_result<'c, E>(
        executor: E,
        (transaction_id, source): (Uuid, FraudRuleResult),
    ) -> Result<
        FraudRuleResult,
        <Self as FraudRuleResultRepository>::AdapterError,
    >
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
}
