use domain::{transaction::Transaction, user::User};
use lib::{
    application::application_result,
    domain::{Id, validation::error::ValidationErrorsWithFields},
};

mod bulk_create;
mod check_user_by_id;
mod create;
mod find_by_id;
mod get_by_id;
mod get_decision_tuple;
mod list;
mod save_decision;

pub use bulk_create::BulkCreateTransactionsUsecase;
pub use create::CreateTransactionUsecase;
pub use find_by_id::FindTransactionByIdUsecase;
pub use get_by_id::GetTransactionByIdUsecase;
pub use list::ListTransactionsUsecase;

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
