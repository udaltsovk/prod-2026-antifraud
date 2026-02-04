use domain::{email::Email, user::User};
use lib::{
    application::application_result,
    domain::{Id, validation::error::ValidationErrorsWithFields},
};

#[derive(thiserror::Error, Debug)]
pub enum UserUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error(transparent)]
    Validation(ValidationErrorsWithFields),

    #[error("Пользователь с таким email уже существует")]
    EmailAlreadyUsed(Email),

    #[error("Пользователь не найден")]
    NotFoundByEmail(Email),

    #[error("Пользователь деактивирован")]
    UserDeactivated,

    #[error("Пользователь не найден")]
    NotFoundById(Id<User>),

    #[error("Неверный пароль")]
    InvalidPassword,

    #[error("Недостаточно прав для выполнения операции")]
    MissingPermissions,
}

application_result!(UserUseCase);
