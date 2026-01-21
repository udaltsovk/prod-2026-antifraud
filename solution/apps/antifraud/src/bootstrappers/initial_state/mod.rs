use std::num::NonZeroU8;

use application::usecase::user::UserUseCase as _;
use domain::user::{CreateUser, role::UserRole};
use lib::async_trait;
use presentation::api::rest::ModulesExt as _;

pub use crate::bootstrappers::initial_state::config::InitialStateConfig;
use crate::{Modules, bootstrappers::BootstrapperExt};

mod config;

pub struct InitialState;

#[async_trait]
impl BootstrapperExt for InitialState {
    type Config = InitialStateConfig;

    async fn bootstrap(config: &Self::Config, modules: Modules) {
        let res: Result<(), String> = async {
            if !modules
                .user_usecase()
                .list(
                    Some(
                        NonZeroU8::try_from(1)
                            .expect("1 is not zero and is in the u8 range"),
                    ),
                    None,
                    Some(&[UserRole::Admin]),
                    None,
                )
                .await
                .map_err(|err| err.to_string())?
                .is_empty()
            {
                return Ok(());
            }

            let user = CreateUser::try_from(&config.admin)
                .map_err(|err| err.to_string())?;

            modules
                .user_usecase()
                .create(user)
                .await
                .map_err(|err| err.to_string())?;

            Ok(())
        }
        .await;

        if let Some(err) = res.err() {
            tracing::error!("Failed to initialize state: {err}");
        }
    }
}
