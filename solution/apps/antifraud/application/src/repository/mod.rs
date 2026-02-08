use crate::repository::{
    fraud_rule::FraudRuleRepository,
    fraud_rule_result::FraudRuleResultRepository,
    statistics::StatisticsRepository, transaction::TransactionRepository,
    user::UserRepository, user_activity::UserActivityRepository,
};

pub mod fraud_rule;
pub mod fraud_rule_result;
pub mod statistics;
pub mod transaction;
pub mod user;
pub mod user_activity;

pub trait RepositoriesModuleExt: Send + Sync {
    fn user(&self) -> &dyn UserRepository;

    fn fraud_rule(&self) -> &dyn FraudRuleRepository;

    fn transaction(&self) -> &dyn TransactionRepository;

    fn fraud_rule_result(&self) -> &dyn FraudRuleResultRepository;

    fn statistics(&self) -> &dyn StatisticsRepository;

    fn user_activity(&self) -> &dyn UserActivityRepository;
}
