use std::fmt::Write as _;

use fromenv::FromEnv;
use lib::{mobc_redis::redis, mobc_sqlx::sqlx::postgres::PgConnectOptions};

#[derive(FromEnv)]
pub struct RepositoriesConfig {
    #[env(nested)]
    pub postgres: PostgresConfig,
    #[env(nested)]
    pub redis: RedisConfig,
}

#[derive(FromEnv)]
#[env(prefix = "DB_")]
pub struct PostgresConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    #[env(default = "5432")]
    pub port: u16,
    #[env(rename = "name")]
    pub database: String,
}

impl From<&PostgresConfig> for PgConnectOptions {
    fn from(config: &PostgresConfig) -> Self {
        Self::new()
            .username(&config.user)
            .password(&config.password)
            .host(&config.host)
            .port(config.port)
            .database(&config.database)
    }
}

#[derive(FromEnv)]
#[env(prefix = "REDIS_")]
pub struct RedisConfig {
    pub host: String,
    pub port: Option<u16>,
}

impl From<&RedisConfig> for redis::Client {
    fn from(config: &RedisConfig) -> Self {
        let url = try {
            let mut url = format!("redis://{}", config.host);

            if let Some(port) = &config.port {
                write!(url, ":{port}")?;
            }

            url
        }
        .expect("url formatting should finish successfully");

        Self::open(url).expect("redis client should open successfully")
    }
}
