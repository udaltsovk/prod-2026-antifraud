use lib::application::application_result;

#[derive(thiserror::Error, Debug)]
pub enum SessionUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),
}

application_result!(SessionUseCase);
