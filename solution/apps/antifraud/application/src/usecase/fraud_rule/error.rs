use domain::fraud_rule::{FraudRule, name::FraudRuleName};
use lib::{
    application::application_result,
    domain::{Id, validation::error::ValidationErrorsWithFields},
};

#[derive(thiserror::Error, Debug)]
pub enum FraudRuleUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error(transparent)]
    Validation(ValidationErrorsWithFields),

    #[error("Правило фрода с таким названием уже существует")]
    NameAlreadyUsed(FraudRuleName),

    #[error("Правило фрода не найдено")]
    NotFoundByName(FraudRuleName),

    #[error("Правило фрода не найдено")]
    NotFoundById(Id<FraudRule>),

    #[error("Недостаточно прав для выполнения операции")]
    MissingPermissions,
}

application_result!(FraudRuleUseCase);
