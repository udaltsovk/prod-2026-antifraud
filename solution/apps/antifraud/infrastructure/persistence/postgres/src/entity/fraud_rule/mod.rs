use chrono::{DateTime, Utc};
use domain::fraud_rule::FraudRule;
use lib::{infrastructure::persistence::entity::DomainTypeFromDb, uuid::Uuid};
use model_mapper::Mapper;
use sqlx::FromRow;

#[derive(Mapper, FromRow, Debug)]
#[mapper(derive(ty = FraudRule, into))]
pub struct StoredFraudRule {
    pub id: Uuid,

    #[mapper(
        when(ty = FraudRule, into_with = DomainTypeFromDb::into_domain),
    )]
    pub name: String,

    #[mapper(
        when(ty = FraudRule, opt(into_with = DomainTypeFromDb::into_domain)),
    )]
    pub description: Option<String>,

    #[mapper(
        when(ty = FraudRule, into_with = DomainTypeFromDb::into_domain),
    )]
    pub dsl_expression: String,

    #[mapper(
        when(ty = FraudRule, into_with = DomainTypeFromDb::into_domain),
    )]
    pub priority: i64,

    #[mapper(
        when(ty = FraudRule, rename = status)
    )]
    pub enabled: bool,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}
