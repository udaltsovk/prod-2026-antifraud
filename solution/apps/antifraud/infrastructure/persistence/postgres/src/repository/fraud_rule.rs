use application::repository::fraud_rule::FraudRuleRepository;
use domain::fraud_rule::{
    CreateFraudRule, FraudRule, description::FraudRuleDescription,
    name::FraudRuleName, status::FraudRuleStatus,
};
use lib::{
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::postgres::error::PostgresAdapterError,
    instrument_all,
    tap::Pipe as _,
};
use sqlx::query_file_as;

use crate::{
    entity::fraud_rule::StoredFraudRule, repository::PostgresRepositoryImpl,
};

#[async_trait]
#[instrument_all]
impl FraudRuleRepository for PostgresRepositoryImpl<FraudRule> {
    type AdapterError = PostgresAdapterError;

    async fn create(
        &self,
        (id, source): (Id<FraudRule>, CreateFraudRule),
    ) -> Result<FraudRule, Self::AdapterError> {
        let id = id.value;
        let name = source.name.into_inner();
        let description = source
            .description
            .map(FraudRuleDescription::into_inner)
            .into_option();
        let dsl_expression = source.dsl_expression.into_inner();
        let enabled = source.status.unwrap_or_default().to_bool();
        let priority: i64 = source.priority.unwrap_or_default().into_inner();

        let mut connection = self.pool.get().await?;

        let fraud_rule = query_file_as!(
            StoredFraudRule,
            "sql/fraud_rule/create.sql",
            id,
            name,
            description,
            dsl_expression,
            enabled,
            priority
        )
        .fetch_one(&mut *connection)
        .await
        .map(FraudRule::from)?;

        Ok(fraud_rule)
    }

    async fn find_by_id(
        &self,
        id: Id<FraudRule>,
    ) -> Result<Option<FraudRule>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        query_file_as!(
            StoredFraudRule,
            "sql/fraud_rule/find_by_id.sql",
            id.value
        )
        .fetch_optional(&mut *connection)
        .await?
        .map(FraudRule::from)
        .pipe(Ok)
    }

    async fn find_by_name(
        &self,
        name: &FraudRuleName,
    ) -> Result<Option<FraudRule>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        query_file_as!(
            StoredFraudRule,
            "sql/fraud_rule/find_by_name.sql",
            name.as_ref()
        )
        .fetch_optional(&mut *connection)
        .await?
        .map(FraudRule::from)
        .pipe(Ok)
    }

    async fn list(
        &self,
        status: Option<FraudRuleStatus>,
    ) -> Result<Vec<FraudRule>, Self::AdapterError> {
        let enabled = status.map(FraudRuleStatus::to_bool);

        let mut connection = self.pool.get().await?;

        query_file_as!(StoredFraudRule, "sql/fraud_rule/list.sql", enabled)
            .fetch_all(&mut *connection)
            .await?
            .into_iter()
            .map(FraudRule::from)
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    async fn update(
        &self,
        source: FraudRule,
    ) -> Result<FraudRule, Self::AdapterError> {
        let id = source.id.value;
        let name = source.name.into_inner();
        let description =
            source.description.map(FraudRuleDescription::into_inner);
        let dsl_expression = source.dsl_expression.into_inner();
        let enabled = source.status.to_bool();
        let priority: i64 = source.priority.into_inner();

        let mut connection = self.pool.get().await?;

        let fraud_rule = query_file_as!(
            StoredFraudRule,
            "sql/fraud_rule/update.sql",
            id,
            name,
            description,
            dsl_expression,
            enabled,
            priority
        )
        .fetch_one(&mut *connection)
        .await
        .map(FraudRule::from)?;

        Ok(fraud_rule)
    }
}
