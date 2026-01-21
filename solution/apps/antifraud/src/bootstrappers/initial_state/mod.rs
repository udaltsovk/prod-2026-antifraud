use application::usecase::user::UserUseCase as _;
use domain::{
    pagination::{Pagination, page::PaginationPage, size::PaginationSize},
    user::{CreateUser, role::UserRole},
};
use lib::{async_trait, tap::Pipe as _};
use presentation::api::rest::ModulesExt as _;

pub use crate::bootstrappers::initial_state::config::InitialStateConfig;
use crate::{Modules, bootstrappers::BootstrapperExt};

mod config;

pub struct InitialState;

impl InitialState {
    async fn bootstrap_fallible(
        config: &<Self as BootstrapperExt>::Config,
        modules: Modules,
    ) -> Result<(), String> {
        let pagination = Pagination {
            page: PaginationPage::default().pipe(Some),
            size: PaginationSize::try_from(1)
                .map_err(|err| err.to_string())?
                .pipe(Some),
        };

        if !modules
            .user_usecase()
            .list(None, Ok(pagination), Some(&[UserRole::Admin]), None)
            .await
            .map_err(|err| err.to_string())?
            .0
            .is_empty()
        {
            return Ok(());
        }

        let user = CreateUser::try_from(&config.admin)
            .map_err(|err| err.to_string())?;

        modules
            .user_usecase()
            .create(None, Ok(user))
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }
}

#[async_trait]
impl BootstrapperExt for InitialState {
    type Config = InitialStateConfig;

    async fn bootstrap(config: &Self::Config, modules: Modules) {
        if let Some(err) = Self::bootstrap_fallible(config, modules).await.err()
        {
            tracing::error!("Failed to initialize state: {err}");
        }
    }
}
