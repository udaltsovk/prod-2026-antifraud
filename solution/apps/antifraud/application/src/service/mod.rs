use crate::service::{
    dsl::DslService, hasher::HasherService, token::TokenService,
};

pub mod dsl;
pub mod hasher;
pub mod token;

pub trait ServicesModuleExt: Send + Sync {
    fn password_hasher(&self) -> &dyn HasherService;

    fn token(&self) -> &dyn TokenService;

    fn dsl(&self) -> &dyn DslService;
}
