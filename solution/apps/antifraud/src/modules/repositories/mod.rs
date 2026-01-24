use application::repository::RepositoriesModuleExt;
use domain::{
    fraud_rule::{FraudRule, result::FraudRuleResult},
    transaction::Transaction,
    user::User,
};
use infrastructure::persistence::postgres::{
    POSTGRES_MIGRATOR, repository::PostgresRepositoryImpl,
};
use lib::{
    infrastructure::persistence::{
        mobc_sqlx::MigratorExt as _, postgres::error::PostgresAdapterError,
        redis::error::RedisAdapterError,
    },
    mobc_redis::{RedisConnectionManager, redis},
    mobc_sqlx::{
        SqlxConnectionManager,
        mobc::Pool,
        sqlx::{Postgres, postgres::PgConnectOptions},
    },
    tap::Pipe as _,
};

use crate::modules::repositories::config::RedisConfig;
pub use crate::modules::repositories::config::{
    PostgresConfig, RepositoriesConfig,
};

mod config;

#[derive(Clone)]
pub struct RepositoriesModule {
    user: PostgresRepositoryImpl<User>,
    fraud_rule: PostgresRepositoryImpl<FraudRule>,
    transaction: PostgresRepositoryImpl<Transaction>,
    fraud_rule_result: PostgresRepositoryImpl<FraudRuleResult>,
}

impl RepositoriesModule {
    pub(crate) async fn new(config: &RepositoriesConfig) -> Self {
        let postgres = Self::setup_postgres(&config.postgres).await;
        let _redis = Self::setup_redis(&config.redis);

        let user_repository = PostgresRepositoryImpl::new(&postgres);
        let fraud_rule_repository = PostgresRepositoryImpl::new(&postgres);
        let transaction_repository = PostgresRepositoryImpl::new(&postgres);
        let fraud_rule_result_repository =
            PostgresRepositoryImpl::new(&postgres);

        Self {
            user: user_repository,
            fraud_rule: fraud_rule_repository,
            transaction: transaction_repository,
            fraud_rule_result: fraud_rule_result_repository,
        }
    }

    async fn setup_postgres(
        config: &PostgresConfig,
    ) -> Pool<SqlxConnectionManager<Postgres>> {
        let postgres = PgConnectOptions::from(config)
            .pipe(SqlxConnectionManager::new)
            .pipe(Pool::new);

        POSTGRES_MIGRATOR.migrate(&postgres).await;

        postgres
    }

    fn setup_redis(config: &RedisConfig) -> Pool<RedisConnectionManager> {
        redis::Client::from(config)
            .pipe(RedisConnectionManager::new)
            .pipe(Pool::new)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error(transparent)]
    Postgres(#[from] PostgresAdapterError),

    #[error(transparent)]
    Redis(#[from] RedisAdapterError),
}

impl RepositoriesModuleExt for RepositoriesModule {
    type Error = RepositoryError;
    type FraudRuleRepository = PostgresRepositoryImpl<FraudRule>;
    type FraudRuleResultRepository = PostgresRepositoryImpl<FraudRuleResult>;
    type TransactionRepository = PostgresRepositoryImpl<Transaction>;
    type UserRepository = PostgresRepositoryImpl<User>;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user
    }

    fn fraud_rule_repository(&self) -> &Self::FraudRuleRepository {
        &self.fraud_rule
    }

    fn transaction_repository(&self) -> &Self::TransactionRepository {
        &self.transaction
    }

    fn fraud_rule_result_repository(&self) -> &Self::FraudRuleResultRepository {
        &self.fraud_rule_result
    }
}
