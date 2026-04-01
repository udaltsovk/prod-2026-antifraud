use crate::service::{
    dsl::DslService, secret_hasher::SecretHasherService, token::TokenService,
};

pub mod dsl;
pub mod secret_hasher;
pub mod token;

pub trait Services = SecretHasherService + TokenService + DslService;
