use std::sync::OnceLock;

use application::repository::{
    fraud_rule::DelegateFraudRuleRepository,
    fraud_rule_result::DelegateFraudRuleResultRepository,
    statistics::DelegateStatisticsRepository,
    transaction::DelegateTransactionRepository, user::DelegateUserRepository,
    user_activity::DelegateUserActivityRepository,
};
use infrastructure::persistence::{
    postgres::{POSTGRES_MIGRATOR, repository::PostgresRepositoryImpl},
    redis::repository::RedisRepositoryImpl,
};
use lib::{
    application::impl_has,
    bootstrap::impl_repositories,
    infrastructure::persistence::{
        mobc_sqlx::MigratorExt as _,
        redis::{Namespace, RedisPool},
        sqlx::SqlxPool,
    },
    mobc_redis::{RedisConnectionManager, redis},
    mobc_sqlx::{
        SqlxConnectionManager,
        mobc::Pool,
        sqlx::{Postgres, postgres::PgConnectOptions},
    },
    tap::Pipe as _,
};

pub use crate::modules::repositories::config::{
    PostgresConfig, RepositoriesConfig,
};
use crate::{Modules, modules::repositories::config::RedisConfig};

mod config;

#[derive(Clone)]
pub struct RepositoriesModule {
    postgres: SqlxPool<Postgres>,
    redis: RedisPool,
}

impl RepositoriesModule {
    pub(crate) async fn new(config: &RepositoriesConfig) -> Self {
        Self {
            postgres: Self::setup_postgres(&config.postgres).await,
            redis: Self::setup_redis(&config.redis),
        }
    }

    async fn setup_postgres(config: &PostgresConfig) -> SqlxPool<Postgres> {
        let postgres = PgConnectOptions::from(config)
            .pipe(SqlxConnectionManager::new)
            .pipe(Pool::new);

        POSTGRES_MIGRATOR.migrate(&postgres).await;

        postgres
    }

    fn setup_redis(config: &RedisConfig) -> RedisPool {
        redis::Client::from(config)
            .pipe(RedisConnectionManager::new)
            .pipe(Pool::new)
    }
}

impl_has! {
    struct: Modules,
    SqlxPool<Postgres>: |s| &s.repositories.postgres,
    RedisPool: |s| &s.repositories.redis,
    Namespace: |_s| {
        static NAMESPACE: OnceLock<Namespace> = OnceLock::new();
        NAMESPACE.get_or_init(|| {
            Namespace::new("antifraud").nest("monolyth")
        })
    }
}

impl_repositories! {
    struct: Modules,
    DelegateUserRepository: PostgresRepositoryImpl,
    DelegateFraudRuleRepository: PostgresRepositoryImpl,
    DelegateTransactionRepository: PostgresRepositoryImpl,
    DelegateFraudRuleResultRepository: PostgresRepositoryImpl,
    DelegateStatisticsRepository: PostgresRepositoryImpl,
    DelegateUserActivityRepository: RedisRepositoryImpl,
}
