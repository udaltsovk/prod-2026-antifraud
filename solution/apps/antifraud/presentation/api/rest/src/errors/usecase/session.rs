use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::session::error::SessionUseCaseError,
};

// use axum::http::StatusCode;

// use serde_json::json;
use crate::ApiError;

impl<R, S> From<SessionUseCaseError<R, S>> for ApiError
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn from(error: SessionUseCaseError<R, S>) -> Self {
        let (status_code, error_code, error, context) = {
            use SessionUseCaseError as E;
            // use StatusCode as C;
            match error {
                E::Repository(_) | E::Service(_) => {
                    Self::internal_server_error(error)
                },
            }
        };

        Self::UseCase {
            status_code,
            error_code,
            message: error,
            context,
        }
    }
}
