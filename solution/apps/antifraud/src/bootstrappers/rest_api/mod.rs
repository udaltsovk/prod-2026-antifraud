use std::net::SocketAddr;

use entrait::Impl;
use lib::{
    async_trait, axum::Router, presentation::api::rest::startup::RestApi,
    tower_http::cors::CorsLayer,
};
use presentation::api::rest::routes;

pub use crate::bootstrappers::rest_api::config::RestApiConfig;
use crate::{Modules, bootstrappers::BootstrapperExt};

mod config;

#[async_trait]
impl BootstrapperExt for RestApi {
    type Config = RestApiConfig;

    async fn bootstrap(config: &Self::Config, modules: &Impl<Modules>) {
        let router = Router::new()
            .nest("/api/{api_version}", routes::router())
            .layer(CorsLayer::very_permissive());

        Self::builder(router, modules)
            .build()
            .run(SocketAddr::from(config))
            .await;
    }
}
