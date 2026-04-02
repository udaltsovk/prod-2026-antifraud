use application::service::{
    dsl::DelegateDslService, secret_hasher::DelegateSecretHasherService,
    token::DelegateTokenService,
};
use infrastructure::services::{
    dsl::DslServiceImplementation, hasher::argon2::Argon2Service,
    token::jwt::JwtService,
};
use lib::{application::impl_has, bootstrap::impl_services};

use crate::Modules;
pub use crate::modules::services::config::ServicesConfig;

mod config;

#[derive(Clone)]
pub struct ServicesModule {
    password_hasher: Argon2Service,
    token: JwtService,
    dsl: DslServiceImplementation,
}

impl ServicesModule {
    pub(crate) fn new(config: &ServicesConfig) -> Self {
        Self {
            password_hasher: Argon2Service::new(),
            token: JwtService::from(&config.jwt),
            dsl: DslServiceImplementation::new(),
        }
    }
}

impl_has! {
    struct: Modules,
    Argon2Service: |s| &s.services.password_hasher,
    JwtService: |s| &s.services.token,
    DslServiceImplementation: |s| &s.services.dsl,
}

impl_services! {
    struct: Modules,
    DelegateSecretHasherService: Argon2Service,
    DelegateTokenService: JwtService,
    DelegateDslService: DslServiceImplementation,
}
