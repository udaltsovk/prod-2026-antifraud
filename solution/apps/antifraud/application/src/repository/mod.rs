use crate::repository::{
    fraud_rule::FraudRuleRepository,
    fraud_rule_result::FraudRuleResultRepository,
    transaction::TransactionRepository, user::UserRepository,
};

pub mod fraud_rule;
pub mod fraud_rule_result;
pub mod transaction;
pub mod user;

pub trait RepositoriesModuleExt: Send + Sync {
    fn user_repository(&self) -> &dyn UserRepository;

    fn fraud_rule_repository(&self) -> &dyn FraudRuleRepository;

    fn transaction_repository(&self) -> &dyn TransactionRepository;

    fn fraud_rule_result_repository(&self) -> &dyn FraudRuleResultRepository;
}
