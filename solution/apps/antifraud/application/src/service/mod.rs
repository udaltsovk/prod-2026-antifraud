use std::fmt::{Debug, Display};

use crate::service::{
    dsl::DslService, hasher::HasherService, token::TokenService,
};

pub mod dsl;
pub mod hasher;
pub mod token;

pub trait ServicesModuleExt: Clone + Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::PasswordHasherService as HasherService>::AdapterError>
        + From<<Self::TokenService as TokenService>::AdapterError>;

    type PasswordHasherService: HasherService + Send + Sync;
    fn password_hasher_service(&self) -> &Self::PasswordHasherService;

    type TokenService: TokenService + Send + Sync;
    fn token_service(&self) -> &Self::TokenService;

    type DslService: DslService + Send + Sync;
    fn dsl_service(&self) -> &Self::DslService;
}
