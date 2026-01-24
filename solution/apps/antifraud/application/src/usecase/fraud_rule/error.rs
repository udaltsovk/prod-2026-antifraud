use domain::fraud_rule::{FraudRule, name::FraudRuleName};
use lib::{
    application::usecase_result,
    domain::{Id, validation::error::ValidationErrors},
};

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

#[derive(thiserror::Error, Debug)]
pub enum FraudRuleUseCaseError<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    #[error("Repository error: {0}")]
    Repository(R::Error),

    #[error(transparent)]
    Service(S::Error),

    #[error(transparent)]
    Validation(ValidationErrors),

    #[error("Правило фрода с таким названием уже существует")]
    NameAlreadyUsed(FraudRuleName),

    #[error("Правило фрода не найдено")]
    NotFoundByName(FraudRuleName),

    #[error("Правило фрода не найдено")]
    NotFoundById(Id<FraudRule>),

    #[error("Недостаточно прав для выполнения операции")]
    MissingPermissions,
}

usecase_result!(FraudRule);
