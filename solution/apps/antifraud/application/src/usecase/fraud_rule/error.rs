use domain::fraud_rule::{FraudRule, name::FraudRuleName};
use lib::{application::application_result, domain::Id};

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
