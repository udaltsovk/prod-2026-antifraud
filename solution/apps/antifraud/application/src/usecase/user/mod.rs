use domain::{
    email::Email,
    user::{User, role::UserRole},
};
use lib::{
    application::application_result,
    domain::{Id, validation::error::ValidationErrorsWithFields},
};

mod authorize;
mod create;
mod deactivate_by_id;
mod find_by_email;
mod find_by_id;
mod get_by_email;
mod get_by_id;
mod list;
mod record_activity;
mod update_by_id;

pub use authorize::AuthorizeUserUsecase;
pub use create::CreateUserUsecase;
pub use deactivate_by_id::DeactivateUserByIdUsecase;
pub use find_by_email::FindUserByEmailUsecase;
pub use find_by_id::FindUserByIdUsecase;
pub use get_by_email::GetUserByEmailUsecase;
pub use get_by_id::GetUserByIdUsecase;
pub use list::ListUsersUsecase;
pub use record_activity::RecordUserActivityUsecase;
pub use update_by_id::UpdateUserByIdUsecase;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CreateUserSource {
    Registration,
    User(UserRole),
}

impl From<UserRole> for CreateUserSource {
    fn from(creator_role: UserRole) -> Self {
        Self::User(creator_role)
    }
}
