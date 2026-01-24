use lib::application::usecase_struct;

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

pub mod fraud_rule;
pub mod session;
pub mod user;

usecase_struct!(RepositoriesModuleExt, ServicesModuleExt);
