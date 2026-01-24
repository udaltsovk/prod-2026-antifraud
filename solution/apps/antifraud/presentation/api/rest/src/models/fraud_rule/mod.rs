use chrono::{DateTime, Utc};
use domain::fraud_rule::{CreateFraudRule, FraudRule, FraudRuleUpdate};
use lib::{
    domain::{into_validators, validation::error::ValidationResult},
    model_mapper::Mapper,
    presentation::api::rest::{UserInput, model::Parseable},
    uuid::Uuid,
};
use serde::{Deserialize, Serialize};

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = FraudRule, from, ignore_extra)]
#[serde(rename_all = "camelCase")]
pub struct JsonFraudRule {
    pub id: Uuid,

    pub name: String,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub dsl_expression: String,

    #[mapper(rename = status)]
    pub enabled: bool,

    pub priority: i64,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateJsonFraudRule {
    #[serde(default)]
    pub name: UserInput<String>,

    #[serde(default)]
    pub description: UserInput<String>,

    #[serde(default)]
    pub dsl_expression: UserInput<String>,

    #[serde(default)]
    pub enabled: UserInput<bool>,

    #[serde(default)]
    pub priority: UserInput<i64>,
}

impl Parseable<CreateFraudRule> for CreateJsonFraudRule {
    const FIELD: &str = "fraudRule";

    fn parse(self) -> ValidationResult<CreateFraudRule> {
        let (errors, (name, description, dsl_expression, status, priority)) = into_validators!(
            self.name,
            self.description,
            self.dsl_expression,
            self.enabled,
            self.priority
        );

        errors.into_result(|ok| CreateFraudRule {
            name: name.validated(ok),
            description: description.validated(ok),
            dsl_expression: dsl_expression.validated(ok),
            status: status.validated(ok),
            priority: priority.validated(ok),
        })
    }
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct JsonFraudRuleUpdate {
    #[serde(default)]
    pub name: UserInput<String>,

    #[serde(default)]
    pub description: UserInput<String>,

    #[serde(default)]
    pub dsl_expression: UserInput<String>,

    #[serde(default)]
    pub enabled: UserInput<bool>,

    #[serde(default)]
    pub priority: UserInput<i64>,
}

impl Parseable<FraudRuleUpdate> for JsonFraudRuleUpdate {
    const FIELD: &str = "fraudRule";

    fn parse(self) -> ValidationResult<FraudRuleUpdate> {
        let (errors, (name, description, dsl_expression, status, priority)) = into_validators!(
            self.name,
            self.description,
            self.dsl_expression,
            self.enabled,
            self.priority
        );

        errors.into_result(|ok| FraudRuleUpdate {
            name: name.validated(ok),
            description: description.validated(ok),
            dsl_expression: dsl_expression.validated(ok),
            status: status.validated(ok),
            priority: priority.validated(ok),
        })
    }
}
