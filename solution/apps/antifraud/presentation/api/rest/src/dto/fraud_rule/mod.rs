use application::service::dsl::DslServiceResult;
use chrono::{DateTime, Utc};
use domain::fraud_rule::{
    CreateFraudRule, FraudRule, FraudRuleUpdate,
    dsl_expression::FraudRuleDslExpression,
};
use lib::{
    domain::DomainType as _,
    model_mapper::Mapper,
    presentation::api::rest::{
        into_validators,
        validation::{
            UserInput, parseable::Parseable, validator::ValidatorResult,
        },
    },
    uuid::Uuid,
};
use serde::{Deserialize, Serialize};

use crate::dto::fraud_rule::dsl_error::DslErrorDto;

pub mod dsl_error;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = FraudRule, from, ignore_extra)]
#[serde(rename_all = "camelCase")]
pub struct FraudRuleDto {
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
pub struct FraudRuleDslExpressionDto {
    #[serde(default)]
    pub dsl_expression: UserInput<String>,
}

impl Parseable<FraudRuleDslExpression> for FraudRuleDslExpressionDto {
    fn parse(self) -> ValidatorResult<FraudRuleDslExpression> {
        let (errors, dsl_expression) = into_validators!(field!(
            self.dsl_expression,
            required,
            "dslExpression"
        ));

        errors.into_result(|ok| dsl_expression.validated(ok))
    }
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateFraudRuleDto {
    #[serde(default)]
    pub name: UserInput<String>,

    #[serde(default)]
    pub description: UserInput<String>,

    #[serde(flatten)]
    pub dsl_expression: FraudRuleDslExpressionDto,

    #[serde(default)]
    pub enabled: UserInput<bool>,

    #[serde(default)]
    pub priority: UserInput<i64>,
}

impl Parseable<CreateFraudRule> for CreateFraudRuleDto {
    fn parse(self) -> ValidatorResult<CreateFraudRule> {
        let (errors, (name, description, dsl_expression, status, priority)) = into_validators!(
            field!(self.name, required, "name"),
            field!(self.description, optional, "description"),
            field!(self.dsl_expression, nested, None),
            field!(self.enabled, optional, "enabled"),
            field!(self.priority, optional, "priority")
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
pub struct FraudRuleUpdateDto {
    #[serde(default)]
    pub name: UserInput<String>,

    #[serde(default)]
    pub description: UserInput<String>,

    #[serde(flatten)]
    pub dsl_expression: FraudRuleDslExpressionDto,

    #[serde(default)]
    pub enabled: UserInput<bool>,

    #[serde(default)]
    pub priority: UserInput<i64>,
}

impl Parseable<FraudRuleUpdate> for FraudRuleUpdateDto {
    fn parse(self) -> ValidatorResult<FraudRuleUpdate> {
        let (errors, (name, description, dsl_expression, status, priority)) = into_validators!(
            field!(self.name, required, "name"),
            field!(self.description, optional, "description"),
            field!(self.dsl_expression, nested, None),
            field!(self.enabled, required, "enabled"),
            field!(self.priority, required, "priority")
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

#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct ValidatedFraudRuleDto {
    pub is_valid: bool,

    pub normalized_expression: Option<String>,

    pub errors: Vec<DslErrorDto>,
}

impl From<DslServiceResult<FraudRuleDslExpression>> for ValidatedFraudRuleDto {
    fn from(result: DslServiceResult<FraudRuleDslExpression>) -> Self {
        let (is_valid, normalized_expression, errors) = match result {
            Ok(normalized) => (true, Some(normalized.into_inner()), vec![]),
            Err(errors) => (
                false,
                None,
                errors.into_iter().map(DslErrorDto::from).collect(),
            ),
        };

        Self {
            is_valid,
            normalized_expression,
            errors,
        }
    }
}
