use std::fmt::{Debug, Display};

use crate::repository::user::UserRepository;

pub mod user;

pub trait RepositoriesModuleExt: Clone + Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::UserRepo as UserRepository>::AdapterError>;

    type UserRepo: UserRepository + Send + Sync;
    fn user_repository(&self) -> &Self::UserRepo;
}
