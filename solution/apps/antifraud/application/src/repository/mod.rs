use std::fmt::{Debug, Display};

use crate::repository::{
    fraud_rule::FraudRuleRepository, user::UserRepository,
};

pub mod fraud_rule;
pub mod user;

pub trait RepositoriesModuleExt: Clone + Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::UserRepository as UserRepository>::AdapterError>
        + From<<Self::FraudRuleRepository as FraudRuleRepository>::AdapterError>;

    type UserRepository: UserRepository + Send + Sync;
    fn user_repository(&self) -> &Self::UserRepository;

    type FraudRuleRepository: FraudRuleRepository + Send + Sync;
    fn fraud_rule_repository(&self) -> &Self::FraudRuleRepository;
}
