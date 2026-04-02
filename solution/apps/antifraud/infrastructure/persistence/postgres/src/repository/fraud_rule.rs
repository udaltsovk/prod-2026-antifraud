use application::repository::fraud_rule::FraudRuleRepositoryImpl;
use domain::fraud_rule::{
    CreateFraudRule, FraudRule, description::FraudRuleDescription,
    name::FraudRuleName, status::FraudRuleStatus,
};
use entrait::entrait;
use lib::{
    anyhow::Result,
    application::di::Has,
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::{HasPoolExt as _, sqlx::SqlxPool},
    instrument_all,
    tap::Pipe as _,
};
use sqlx::{Postgres, query_file_as};

use crate::{
    entity::fraud_rule::StoredFraudRule, repository::PostgresRepositoryImpl,
};

#[entrait]
#[async_trait]
#[instrument_all]
impl FraudRuleRepositoryImpl for PostgresRepositoryImpl {
    async fn create_fraud_rule<Deps>(
        deps: &Deps,
        (id, source): (Id<FraudRule>, CreateFraudRule),
    ) -> Result<FraudRule>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let id = id.value;
        let name = source.name.into_inner();
        let description =
            source.description.map(FraudRuleDescription::into_inner);
        let dsl_expression = source.dsl_expression.into_inner();
        let enabled = source.status.unwrap_or_default().to_bool();
        let priority: i64 = source.priority.unwrap_or_default().into_inner();

        let mut connection = deps.get_connection().await?;

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

    async fn find_fraud_rule_by_id<Deps>(
        deps: &Deps,
        id: Id<FraudRule>,
    ) -> Result<Option<FraudRule>>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let mut connection = deps.get_connection().await?;

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

    async fn find_fraud_rule_by_name<Deps>(
        deps: &Deps,
        name: &FraudRuleName,
    ) -> Result<Option<FraudRule>>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let mut connection = deps.get_connection().await?;

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

    async fn list_fraud_rules<Deps>(
        deps: &Deps,
        status: Option<FraudRuleStatus>,
    ) -> Result<Vec<FraudRule>>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let enabled = status.map(FraudRuleStatus::to_bool);

        let mut connection = deps.get_connection().await?;

        query_file_as!(StoredFraudRule, "sql/fraud_rule/list.sql", enabled)
            .fetch_all(&mut *connection)
            .await?
            .into_iter()
            .map(FraudRule::from)
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    async fn update_fraud_rule<Deps>(
        deps: &Deps,
        source: FraudRule,
    ) -> Result<FraudRule>
    where
        Deps: Has<SqlxPool<Postgres>>,
    {
        let id = source.id.value;
        let name = source.name.into_inner();
        let description =
            source.description.map(FraudRuleDescription::into_inner);
        let dsl_expression = source.dsl_expression.into_inner();
        let enabled = source.status.to_bool();
        let priority: i64 = source.priority.into_inner();

        let mut connection = deps.get_connection().await?;

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
