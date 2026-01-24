use std::sync::Arc;

use application::service::ServicesModuleExt;
use infrastructure::services::{
    dsl::DslServiceImpl,
    hasher::argon2::{Argon2AdapterError, Argon2Service},
    token::jwt::{JwtAdapterError, JwtService},
};

pub use crate::modules::services::config::ServicesConfig;

mod config;

#[derive(Clone)]
pub struct ServicesModule {
    password_hasher: Arc<Argon2Service>,
    token: Arc<JwtService>,
    dsl: Arc<DslServiceImpl>,
}

impl ServicesModule {
    pub(crate) fn new(config: &ServicesConfig) -> Self {
        Self {
            password_hasher: Arc::new(Argon2Service::new()),
            token: Arc::from(&config.jwt),
            dsl: Arc::new(DslServiceImpl::new()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Argon2 service error: {0}")]
    Argon2(#[from] Argon2AdapterError),

    #[error("JWT service error: {0}")]
    Jwt(#[from] JwtAdapterError),
}

impl ServicesModuleExt for ServicesModule {
    type DslService = DslServiceImpl;
    type Error = ServiceError;
    type PasswordHasherService = Argon2Service;
    type TokenService = JwtService;

    fn password_hasher_service(&self) -> &Self::PasswordHasherService {
        &self.password_hasher
    }

    fn token_service(&self) -> &Self::TokenService {
        &self.token
    }

    fn dsl_service(&self) -> &Self::DslService {
        &self.dsl
    }
}
