use chrono::{DateTime, Utc};
use lib::domain::{Id, validation::Optional};

use crate::fraud_rule::{
    description::FraudRuleDescription,
    dsl_expression::FraudRuleDslExpression,
    name::FraudRuleName,
    priority::FraudRulePriority,
    result::{
        FraudRuleResult, description::FraudRuleResultDescription,
        status::FraudRuleResultStatus,
    },
    status::FraudRuleStatus,
};

pub mod description;
pub mod dsl_expression;
pub mod name;
pub mod priority;
pub mod result;
pub mod status;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FraudRule {
    pub id: Id<Self>,
    pub name: FraudRuleName,
    pub description: Option<FraudRuleDescription>,
    pub dsl_expression: FraudRuleDslExpression,
    pub status: FraudRuleStatus,
    pub priority: FraudRulePriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FraudRule {
    pub fn apply<F>(&self, f: F) -> FraudRuleResult
    where
        F: FnOnce(
            &FraudRuleDslExpression,
        )
            -> (FraudRuleResultStatus, FraudRuleResultDescription),
    {
        let Self {
            id,
            name,
            dsl_expression,
            priority,
            ..
        } = self;

        let (status, description) = f(dsl_expression);

        FraudRuleResult {
            rule_id: *id,
            rule_name: name.clone(),
            priority: *priority,
            status,
            description,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CreateFraudRule {
    pub name: FraudRuleName,
    pub description: Optional<FraudRuleDescription>,
    pub dsl_expression: FraudRuleDslExpression,
    pub status: Optional<FraudRuleStatus>,
    pub priority: Optional<FraudRulePriority>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FraudRuleUpdate {
    pub name: FraudRuleName,
    pub description: Optional<FraudRuleDescription>,
    pub dsl_expression: FraudRuleDslExpression,
    pub status: FraudRuleStatus,
    pub priority: FraudRulePriority,
}

impl PartialEq<FraudRule> for FraudRuleUpdate {
    #[expect(
        clippy::suspicious_operation_groupings,
        reason = "I'm sure there's no mistake"
    )]
    fn eq(&self, other: &FraudRule) -> bool {
        let FraudRule {
            name: current_name,
            description: current_description,
            dsl_expression: current_dsl_expression,
            status: current_status,
            priority: current_priority,
            ..
        } = other;

        let Self {
            name: new_name,
            description: new_description,
            dsl_expression: new_dsl_expression,
            status: new_status,
            priority: new_priority,
        } = self;

        current_name == new_name
            && new_description.as_option() == current_description.as_ref()
            && new_dsl_expression == current_dsl_expression
            && new_status == current_status
            && new_priority == current_priority
    }
}

impl FraudRuleUpdate {
    #[must_use]
    #[expect(
        clippy::needless_pass_by_value,
        reason = "We want to consume update instance here"
    )]
    pub fn apply_to(self, fraud_rule: FraudRule) -> FraudRule {
        let FraudRule {
            id: current_id,
            created_at: current_created_at,
            updated_at: current_updated_at,
            ..
        } = fraud_rule;

        let Self {
            name: new_name,
            description: new_description,
            dsl_expression: new_dsl_expression,
            status: new_status,
            priority: new_priority,
        } = self;

        FraudRule {
            id: current_id,
            name: new_name,
            description: new_description.into(),
            dsl_expression: new_dsl_expression,
            status: new_status,
            priority: new_priority,
            created_at: current_created_at,
            updated_at: current_updated_at,
        }
    }
}
