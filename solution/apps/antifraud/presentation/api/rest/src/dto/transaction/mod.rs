use std::net::IpAddr;

use chrono::{DateTime, Utc};
use domain::transaction::{CreateTransaction, Transaction};
use lib::{
    domain::DomainType,
    model_mapper::Mapper,
    presentation::api::rest::{
        into_validators,
        validation::{
            UserInput, parseable::Parseable, validator::ValidatorResult,
        },
    },
    uuid::Uuid,
};
use serde::{Deserialize, Serialize};

use crate::dto::transaction::{
    channel::TransactionChannelDto,
    location::{CreateTransactionLocationDto, TransactionLocationDto},
    status::TransactionStatusDto,
};

pub mod channel;
pub mod decision;
pub mod location;
pub mod pagination;
pub mod status;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = Transaction, from)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDto {
    pub id: Uuid,

    #[mapper(with = DomainType::into_inner)]
    pub user_id: Uuid,

    pub amount: f64,

    pub currency: String,

    pub status: TransactionStatusDto,

    #[mapper(skip(default(value = status.is_fraudulent())))]
    pub is_fraud: bool,

    #[mapper(opt)]
    pub merchant_id: Option<String>,

    #[mapper(opt)]
    pub merchant_category_code: Option<String>,

    pub timestamp: DateTime<Utc>,

    #[mapper(opt)]
    pub ip_address: Option<IpAddr>,

    #[mapper(opt)]
    pub device_id: Option<String>,

    #[mapper(opt)]
    pub channel: Option<TransactionChannelDto>,

    pub location: TransactionLocationDto,

    #[mapper(opt)]
    pub metadata: Option<serde_json::Value>,

    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionDto {
    #[serde(default)]
    pub user_id: UserInput<Uuid>,

    #[serde(default)]
    pub amount: UserInput<f64>,

    #[serde(default)]
    pub currency: UserInput<String>,

    #[serde(default)]
    pub merchant_id: UserInput<String>,

    #[serde(default)]
    pub merchant_category_code: UserInput<String>,

    #[serde(default)]
    pub timestamp: UserInput<DateTime<Utc>>,

    #[serde(default)]
    pub ip_address: UserInput<IpAddr>,

    #[serde(default)]
    pub device_id: UserInput<String>,

    #[serde(default)]
    pub channel: UserInput<String>,

    #[serde(default)]
    pub location: UserInput<CreateTransactionLocationDto>,

    #[serde(default)]
    pub metadata: UserInput<serde_value::Value>,
}

impl Parseable<CreateTransaction> for CreateTransactionDto {
    fn parse(self) -> ValidatorResult<CreateTransaction> {
        let (
            errors,
            (
                user_id,
                amount,
                currency,
                merchant_id,
                merchant_category_code,
                timestamp,
                ip_address,
                device_id,
                channel,
                location,
                metadata,
            ),
        ) = into_validators!(
            field!(self.user_id, required, "userId"),
            field!(self.amount, required, "amount"),
            field!(self.currency, required, "currency"),
            field!(self.merchant_id, optional, "merchantId"),
            field!(
                self.merchant_category_code,
                optional,
                "merchantCategoryCode"
            ),
            field!(self.timestamp, required, "timestamp"),
            field!(self.ip_address, optional, "ipAddress"),
            field!(self.device_id, optional, "deviceId"),
            field!(self.channel, optional, "channel"),
            field!(self.location, nested, "location"),
            field!(self.metadata, optional, "metadata")
        );

        errors.into_result(|ok| CreateTransaction {
            user_id: user_id.validated(ok),
            amount: amount.validated(ok),
            currency: currency.validated(ok),
            merchant_id: merchant_id.validated(ok),
            merchant_category_code: merchant_category_code.validated(ok),
            timestamp: timestamp.validated(ok),
            ip_address: ip_address.validated(ok),
            device_id: device_id.validated(ok),
            channel: channel.validated(ok),
            location: location.validated(ok),
            metadata: metadata.validated(ok),
        })
    }
}
