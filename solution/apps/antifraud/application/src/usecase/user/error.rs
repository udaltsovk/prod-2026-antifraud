use domain::{email::Email, user::User};
use lib::{
    application::usecase_result,
    domain::{Id, validation::error::ValidationErrors},
};

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

#[derive(thiserror::Error, Debug)]
pub enum UserUseCaseError<R, S>
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

    #[error("Пользователь с таким email уже существует")]
    EmailAlreadyUsed(Email),

    #[error("Пользователь не найден")]
    NotFoundByEmail { email: Email, from_auth: bool },

    #[error("Пользователь деактивирован")]
    UserDeactivated,

    #[error("Пользователь не найден")]
    NotFoundById(Id<User>),

    #[error("Неверный пароль")]
    InvalidPassword,

    #[error("Недостаточно прав для выполнения операции")]
    MissingPermissions,
}

usecase_result!(User);
