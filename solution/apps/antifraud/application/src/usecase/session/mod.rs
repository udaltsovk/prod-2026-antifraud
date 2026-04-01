use lib::application::application_result;

mod create;
mod get_from_token;

pub use create::CreateSessionUsecase;
pub use get_from_token::GetSessionFromTokenUsecase;

#[derive(thiserror::Error, Debug)]
pub enum SessionUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),
}

application_result!(SessionUseCase);
