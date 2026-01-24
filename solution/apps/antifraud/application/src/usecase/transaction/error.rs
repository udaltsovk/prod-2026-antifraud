use domain::{transaction::Transaction, user::User};
use lib::{
    application::application_result,
    domain::{Id, validation::error::ValidationErrors},
};

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

#[derive(thiserror::Error, Debug)]
pub enum TransactionUseCaseError<R, S>
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

    #[error("Пользователь деактивирован")]
    UserDeactivated,

    #[error("Пользователь не найдена")]
    UserNotFoundById(Id<User>),

    #[error("Транзакция не найдена")]
    TransactionNotFoundById(Id<Transaction>),

    #[error("Недостаточно прав для выполнения операции")]
    MissingPermissions,
}

application_result!(TransactionUseCase<R, S>);
