use lib::application::application_result;

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

#[derive(thiserror::Error, Debug)]
pub enum SessionUseCaseError<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    #[error("Repository error: {0}")]
    Repository(R::Error),

    #[error(transparent)]
    Service(S::Error),
}

application_result!(SessionUseCase<R, S>);
