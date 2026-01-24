use lib::infrastructure::persistence::repository_impl_struct;
use mobc_sqlx::{SqlxConnectionManager, mobc};
use sqlx::Postgres;

mod fraud_rule;
mod fraud_rule_result;
mod transaction;
mod user;

repository_impl_struct!(Postgres, SqlxConnectionManager<Postgres>);
