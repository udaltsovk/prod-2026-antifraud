use std::net::IpAddr;

use chrono::{DateTime, Utc};
use domain::transaction::Transaction;
use lib::{infrastructure::persistence::entity::DomainTypeFromDb, uuid::Uuid};
use model_mapper::Mapper;
use serde_json::Value;
use sqlx::FromRow;

use crate::entity::transaction::{
    channel::StoredTransactionChannel, location::StoredTransactionLocation,
    verdict::StoredTransactionVerdict,
};

pub mod channel;
pub mod location;
pub mod verdict;

#[derive(Mapper, FromRow, Debug)]
#[mapper(derive(ty = Transaction, into))]
pub struct StoredTransaction {
    pub id: Uuid,

    #[mapper(
        when(ty = Transaction, into_with = DomainTypeFromDb::into_domain),
    )]
    pub user_id: Uuid,

    #[mapper(
        when(ty = Transaction, into_with = DomainTypeFromDb::into_domain),
    )]
    pub amount: f64,

    #[mapper(
        when(ty = Transaction, into_with = DomainTypeFromDb::into_domain),
    )]
    pub currency: String,

    #[mapper(
        when(ty = Transaction, rename = status),
    )]
    pub verdict: StoredTransactionVerdict,

    #[mapper(
        when(ty = Transaction, opt(into_with = DomainTypeFromDb::into_domain)),
    )]
    pub merchant_id: Option<String>,

    #[mapper(
        when(ty = Transaction, opt(into_with = DomainTypeFromDb::into_domain)),
    )]
    pub merchant_category_code: Option<String>,

    #[mapper(
        when(ty = Transaction, rename = timestamp, into_with = DomainTypeFromDb::into_domain),
    )]
    pub specified_timestamp: DateTime<Utc>,

    #[mapper(
        when(ty = Transaction, opt(into_with = DomainTypeFromDb::into_domain)),
    )]
    pub ip_address: Option<IpAddr>,

    #[mapper(
        when(ty = Transaction, opt(into_with = DomainTypeFromDb::into_domain)),
    )]
    pub device_id: Option<String>,

    #[mapper(
        when(ty = Transaction, opt),
    )]
    pub channel: Option<StoredTransactionChannel>,

    pub location: StoredTransactionLocation,

    #[mapper(
        when(ty = Transaction, opt(into_with = DomainTypeFromDb::into_domain)),
    )]
    pub metadata: Option<Value>,

    pub created_at: DateTime<Utc>,
}
