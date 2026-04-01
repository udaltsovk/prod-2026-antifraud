use application::usecase::user::{
    CreateUserSource, CreateUserUsecase, FindUserByEmailUsecase,
};
use domain::user::CreateUser;
use entrait::Impl;
use lib::{anyhow::Result, async_trait};

pub use crate::bootstrappers::initial_state::config::InitialStateConfig;
use crate::{Modules, bootstrappers::BootstrapperExt};

mod config;

pub struct InitialState;

impl InitialState {
    async fn bootstrap_fallible<App>(
        app: &App,
        config: &<Self as BootstrapperExt>::Config,
    ) -> Result<()>
    where
        App: Send + Sync + FindUserByEmailUsecase + CreateUserUsecase,
    {
        let user = CreateUser::try_from(&config.admin)?;

        if app.find_user_by_email(&user.email).await?.is_some() {
            return Ok(());
        }

        app.create_user(CreateUserSource::Registration, Ok(user))
            .await?;

        Ok(())
    }
}

#[async_trait]
impl BootstrapperExt for InitialState {
    type Config = InitialStateConfig;

    async fn bootstrap(config: &Self::Config, modules: &Impl<Modules>) {
        if let Some(err) = Self::bootstrap_fallible(modules, config).await.err()
        {
            tracing::error!("Failed to initialize state: {err}");
        }
    }
}
