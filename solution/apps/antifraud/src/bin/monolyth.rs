use antifraud::{
    AppConfig, Modules, bootstrappers::initial_state::InitialState,
};
use lib::{
    bootstrap::{
        ConfigExt as _, bootstrap, configure_jemalloc, instrumentation::stdout,
    },
    presentation::api::rest::startup::RestApi,
};

configure_jemalloc!();

#[tokio::main]
async fn main() {
    let config = AppConfig::load();

    stdout::wrap(bootstrap!(
        antifraud,
        [InitialState(&config.initial_state), RestApi(&config.server)],
        Modules::init(&config.modules)
    ))
    .await;
}
