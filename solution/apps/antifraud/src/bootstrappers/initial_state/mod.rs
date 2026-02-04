use application::usecase::user::{CreateUserSource, UserUseCase as _};
use domain::user::CreateUser;
use lib::{anyhow::Result, async_trait};
use presentation::api::rest::ModulesExt as _;

pub use crate::bootstrappers::initial_state::config::InitialStateConfig;
use crate::{Modules, bootstrappers::BootstrapperExt};

mod config;

pub struct InitialState;

impl InitialState {
    async fn bootstrap_fallible(
        config: &<Self as BootstrapperExt>::Config,
        modules: Modules,
    ) -> Result<()> {
        let user = CreateUser::try_from(&config.admin)?;

        if modules
            .user_usecase()
            .find_by_email(&user.email)
            .await?
            .is_some()
        {
            return Ok(());
        }

        modules
            .user_usecase()
            .create(CreateUserSource::Registration, Ok(user))
            .await?;

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
