use application::usecase::{
    fraud_rule::FraudRuleUseCase, session::SessionUseCase,
    statistics::StatisticsUseCase, transaction::TransactionUseCase,
    user::UserUseCase,
};

pub trait ModulesExt: Clone + Send + Sync + 'static {
    fn user_usecase(&self) -> &impl UserUseCase;

    fn session_usecase(&self) -> &impl SessionUseCase;

    fn fraud_rule_usecase(&self) -> &impl FraudRuleUseCase;

    fn transaction_usecase(&self) -> &impl TransactionUseCase;

    fn statistics_usecase(&self) -> &impl StatisticsUseCase;
}
