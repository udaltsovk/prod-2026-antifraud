use std::fmt::{Debug, Display};

use crate::repository::{
    fraud_rule::FraudRuleRepository,
    fraud_rule_result::FraudRuleResultRepository,
    transaction::TransactionRepository, user::UserRepository,
};

pub mod fraud_rule;
pub mod fraud_rule_result;
pub mod transaction;
pub mod user;

pub trait RepositoriesModuleExt: Clone + Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::UserRepository as UserRepository>::AdapterError>
        + From<<Self::FraudRuleRepository as FraudRuleRepository>::AdapterError>
        + From<<Self::TransactionRepository as TransactionRepository>::AdapterError>
        + From<<Self::FraudRuleResultRepository as FraudRuleResultRepository>::AdapterError>;

    type UserRepository: UserRepository + Send + Sync;
    fn user_repository(&self) -> &Self::UserRepository;

    type FraudRuleRepository: FraudRuleRepository + Send + Sync;
    fn fraud_rule_repository(&self) -> &Self::FraudRuleRepository;

    type TransactionRepository: TransactionRepository + Send + Sync;
    fn transaction_repository(&self) -> &Self::TransactionRepository;

    type FraudRuleResultRepository: FraudRuleResultRepository + Send + Sync;
    fn fraud_rule_result_repository(&self) -> &Self::FraudRuleResultRepository;
}
