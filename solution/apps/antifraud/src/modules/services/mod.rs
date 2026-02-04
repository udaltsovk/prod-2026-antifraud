use application::service::{
    ServicesModuleExt, dsl::DslService, hasher::HasherService,
    token::TokenService,
};
use infrastructure::services::{
    dsl::DslServiceImpl, hasher::argon2::Argon2Service, token::jwt::JwtService,
};

pub use crate::modules::services::config::ServicesConfig;

mod config;

#[derive(Clone)]
pub struct ServicesModule {
    password_hasher: Argon2Service,
    token: JwtService,
    dsl: DslServiceImpl,
}

impl ServicesModule {
    pub(crate) fn new(config: &ServicesConfig) -> Self {
        Self {
            password_hasher: Argon2Service::new(),
            token: JwtService::from(&config.jwt),
            dsl: DslServiceImpl::new(),
        }
    }
}

impl ServicesModuleExt for ServicesModule {
    fn password_hasher_service(&self) -> &dyn HasherService {
        &self.password_hasher
    }

    fn token_service(&self) -> &dyn TokenService {
        &self.token
    }

    fn dsl_service(&self) -> &dyn DslService {
        &self.dsl
    }
}
