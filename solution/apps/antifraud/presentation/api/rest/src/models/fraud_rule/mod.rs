use application::service::dsl::DslServiceResult;
use chrono::{DateTime, Utc};
use domain::fraud_rule::{
    CreateFraudRule, FraudRule, FraudRuleUpdate,
    dsl_expression::FraudRuleDslExpression,
};
use lib::{
    domain::{
        DomainType as _, into_validators, validation::error::ValidationResult,
    },
    model_mapper::Mapper,
    presentation::api::rest::{
        UserInput, into_nested_validators, model::Parseable,
    },
    uuid::Uuid,
};
use serde::{Deserialize, Serialize};

use crate::models::fraud_rule::dsl_error::JsonDslError;

pub mod dsl_error;

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
pub struct JsonFraudRuleDslExpression {
    #[serde(default)]
    pub dsl_expression: UserInput<String>,
}

impl Parseable<FraudRuleDslExpression> for JsonFraudRuleDslExpression {
    const FIELD: &str = "dslExpression";

    fn parse(self) -> ValidationResult<FraudRuleDslExpression> {
        let (errors, dsl_expression) = into_validators!(self.dsl_expression);

        errors.into_result(|ok| dsl_expression.validated(ok))
    }
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateJsonFraudRule {
    #[serde(default)]
    pub name: UserInput<String>,

    #[serde(default)]
    pub description: UserInput<String>,

    #[serde(flatten)]
    pub dsl_expression: JsonFraudRuleDslExpression,

    #[serde(default)]
    pub enabled: UserInput<bool>,

    #[serde(default)]
    pub priority: UserInput<i64>,
}

impl Parseable<CreateFraudRule> for CreateJsonFraudRule {
    const FIELD: &str = "fraudRule";

    fn parse(self) -> ValidationResult<CreateFraudRule> {
        let (mut errors, (name, description, status, priority)) = into_validators!(
            self.name,
            self.description,
            self.enabled,
            self.priority
        );

        let (nested_errors, dsl_expression) =
            into_nested_validators!(self.dsl_expression);

        errors.extend(nested_errors);

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

    #[serde(flatten)]
    pub dsl_expression: JsonFraudRuleDslExpression,

    #[serde(default)]
    pub enabled: UserInput<bool>,

    #[serde(default)]
    pub priority: UserInput<i64>,
}

impl Parseable<FraudRuleUpdate> for JsonFraudRuleUpdate {
    const FIELD: &str = "fraudRule";

    fn parse(self) -> ValidationResult<FraudRuleUpdate> {
        let (mut errors, (name, description, status, priority)) = into_validators!(
            self.name,
            self.description,
            self.enabled,
            self.priority
        );

        let (nested_errors, dsl_expression) =
            into_nested_validators!(self.dsl_expression);

        errors.extend(nested_errors);

        errors.into_result(|ok| FraudRuleUpdate {
            name: name.validated(ok),
            description: description.validated(ok),
            dsl_expression: dsl_expression.validated(ok),
            status: status.validated(ok),
            priority: priority.validated(ok),
        })
    }
}

#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct ValidatedJsonFraudRule {
    pub is_valid: bool,

    pub normalized_expression: Option<String>,

    pub errors: Vec<JsonDslError>,
}

impl From<DslServiceResult<FraudRuleDslExpression>> for ValidatedJsonFraudRule {
    fn from(result: DslServiceResult<FraudRuleDslExpression>) -> Self {
        let (is_valid, normalized_expression, errors) = match result {
            Ok(normalized) => (true, Some(normalized.into_inner()), vec![]),
            Err(errors) => (
                false,
                None,
                errors.into_iter().map(JsonDslError::from).collect(),
            ),
        };

        Self {
            is_valid,
            normalized_expression,
            errors,
        }
    }
}
