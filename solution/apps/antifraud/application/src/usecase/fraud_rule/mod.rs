use domain::fraud_rule::{FraudRule, name::FraudRuleName};
use lib::{application::application_result, domain::Id};

mod create;
mod disable_by_id;
mod find_by_id;
mod find_by_name;
mod get_by_id;
mod get_by_name;
mod list;
mod normalize_dsl_expression;
mod update_by_id;

pub use create::CreateFraudRuleUsecase;
pub use disable_by_id::DisableFraudRuleByIdUsecase;
pub use find_by_id::FindFraudRuleByIdUsecase;
pub use find_by_name::FindFraudRuleByNameUsecase;
pub use get_by_id::GetFraudRuleByIdUsecase;
pub use get_by_name::GetFraudRuleByNameUsecase;
pub use list::ListFraudRulesUsecase;
pub use normalize_dsl_expression::NormalizeDslExpressionUsecase;
pub use update_by_id::UpdateFraudRuleByIdUsecase;

#[derive(thiserror::Error, Debug)]
pub enum FraudRuleUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error("Правило фрода с таким названием уже существует")]
    NameAlreadyUsed(FraudRuleName),

    #[error("Правило фрода не найдено")]
    NotFoundByName(FraudRuleName),

    #[error("Правило фрода не найдено")]
    NotFoundById(Id<FraudRule>),
}

application_result!(FraudRuleUseCase);
