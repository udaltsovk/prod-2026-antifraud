use lib::infrastructure::persistence::repository_impl_struct;

mod fraud_rule;
mod fraud_rule_result;
mod statistics;
mod transaction;
mod user;

repository_impl_struct!(Postgres);
