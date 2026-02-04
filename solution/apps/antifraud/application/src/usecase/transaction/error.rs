use domain::{transaction::Transaction, user::User};
use lib::{
    application::application_result,
    domain::{Id, validation::error::ValidationErrorsWithFields},
};

#[derive(thiserror::Error, Debug)]
pub enum TransactionUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error(transparent)]
    Validation(ValidationErrorsWithFields),

    #[error("Пользователь деактивирован")]
    UserDeactivated,

    #[error("Пользователь не найден")]
    UserNotFoundById(Id<User>),

    #[error("Транзакция не найдена")]
    TransactionNotFoundById(Id<Transaction>),

    #[error("Недостаточно прав для выполнения операции")]
    MissingPermissions,
}

application_result!(TransactionUseCase);
