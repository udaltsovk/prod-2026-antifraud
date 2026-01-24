use application::repository::RepositoriesModuleExt;
use domain::{fraud_rule::FraudRule, user::User};
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
    user_repository: PostgresRepositoryImpl<User>,
    fraud_rule_repository: PostgresRepositoryImpl<FraudRule>,
}

impl RepositoriesModule {
    pub(crate) async fn new(config: &RepositoriesConfig) -> Self {
        let postgres = Self::setup_postgres(&config.postgres).await;
        let _redis = Self::setup_redis(&config.redis);

        let user_repository = PostgresRepositoryImpl::new(&postgres);
        let fraud_rule_repository = PostgresRepositoryImpl::new(&postgres);

        Self {
            user_repository,
            fraud_rule_repository,
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
    type UserRepository = PostgresRepositoryImpl<User>;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }

    fn fraud_rule_repository(&self) -> &Self::FraudRuleRepository {
        &self.fraud_rule_repository
    }
}
