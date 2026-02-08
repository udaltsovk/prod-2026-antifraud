use domain::user::User;
use lib::{
    application::application_result,
    domain::{Id, validation::error::ValidationErrorsWithFields},
};

#[derive(thiserror::Error, Debug)]
pub enum StatisticsUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error(transparent)]
    Validation(ValidationErrorsWithFields),

    #[error("Пользователь не найден")]
    UserNotFoundById(Id<User>),

    #[error("Недостаточно прав для выполнения операции")]
    MissingPermissions,
}

application_result!(StatisticsUseCase);
