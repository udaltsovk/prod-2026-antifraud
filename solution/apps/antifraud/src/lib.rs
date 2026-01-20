pub mod bootstrappers;
mod modules;

use fromenv::FromEnv;
pub use modules::Modules;

use crate::{
    bootstrappers::{
        initial_state::InitialStateConfig, rest_api::RestApiConfig,
    },
    modules::ModulesConfig,
};

#[derive(FromEnv)]
pub struct AppConfig {
    #[env(nested)]
    pub modules: ModulesConfig,
    #[env(nested)]
    pub server: RestApiConfig,
    #[env(nested)]
    pub initial_state: InitialStateConfig,
}
