use application::usecase::{
    UseCase, fraud_rule::FraudRuleUseCase, session::SessionUseCase,
    transaction::TransactionUseCase, user::UserUseCase,
};
use domain::{
    fraud_rule::FraudRule, session::Session, transaction::Transaction,
    user::User,
};
use presentation::api::rest::ModulesExt;

pub use crate::modules::config::ModulesConfig;
use crate::modules::{
    repositories::RepositoriesModule, services::ServicesModule,
};

mod config;
mod repositories;
mod services;

#[derive(Clone)]
pub struct Modules {
    #[expect(dead_code, reason = "We might need that in the future")]
    repositories_module: RepositoriesModule,
    #[expect(dead_code, reason = "We might need that in the future")]
    services_module: ServicesModule,
    user_usecase: UseCase<User>,
    session_usecase: UseCase<Session>,
    fraud_rule_usecase: UseCase<FraudRule>,
    transaction_usecase: UseCase<Transaction>,
}

impl ModulesExt for Modules {
    fn user_usecase(&self) -> &impl UserUseCase {
        &self.user_usecase
    }

    fn session_usecase(&self) -> &impl SessionUseCase {
        &self.session_usecase
    }

    fn fraud_rule_usecase(&self) -> &impl FraudRuleUseCase {
        &self.fraud_rule_usecase
    }

    fn transaction_usecase(&self) -> &impl TransactionUseCase {
        &self.transaction_usecase
    }
}

impl Modules {
    pub async fn init(config: &ModulesConfig) -> Self {
        let repositories_module =
            RepositoriesModule::new(&config.repositories).await;
        let services_module = ServicesModule::new(&config.services);

        let user_usecase = UseCase::new(&repositories_module, &services_module);
        let session_usecase =
            UseCase::new(&repositories_module, &services_module);
        let fraud_rule_usecase =
            UseCase::new(&repositories_module, &services_module);
        let transaction_usecase =
            UseCase::new(&repositories_module, &services_module);

        Self {
            repositories_module,
            services_module,
            user_usecase,
            session_usecase,
            fraud_rule_usecase,
            transaction_usecase,
        }
    }
}
