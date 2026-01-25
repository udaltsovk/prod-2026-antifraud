use std::net::IpAddr;

use chrono::{DateTime, Utc};
use domain::transaction::{CreateTransaction, Transaction};
use lib::{
    domain::{
        DomainType, into_validators, validation::error::ValidationResult,
    },
    model_mapper::Mapper,
    presentation::api::rest::{
        UserInput, into_nested_validators, model::Parseable,
    },
    uuid::Uuid,
};
use serde::{Deserialize, Serialize};

use crate::models::transaction::{
    channel::JsonTransactionChannel,
    location::{CreateJsonTransactionLocation, JsonTransactionLocation},
    status::JsonTransactionStatus,
};

pub mod channel;
pub mod decision;
pub mod location;
pub mod pagination;
pub mod status;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = Transaction, from)]
#[serde(rename_all = "UPPERCASE")]
pub struct JsonTransaction {
    pub id: Uuid,

    #[mapper(with = DomainType::into_inner)]
    pub user_id: Uuid,

    pub amount: f64,

    pub currency: String,

    pub status: JsonTransactionStatus,

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
    pub channel: Option<JsonTransactionChannel>,

    pub location: JsonTransactionLocation,

    #[mapper(opt)]
    pub metadata: Option<serde_json::Value>,

    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "UPPERCASE")]
pub struct CreateJsonTransaction {
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
    pub location: CreateJsonTransactionLocation,

    #[serde(default)]
    pub metadata: UserInput<serde_value::Value>,
}

impl Parseable<CreateTransaction> for CreateJsonTransaction {
    const FIELD: &str = "transaction";

    fn parse(self) -> ValidationResult<CreateTransaction> {
        let (
            mut errors,
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
                metadata,
            ),
        ) = into_validators!(
            self.user_id,
            self.amount,
            self.currency,
            self.merchant_id,
            self.merchant_category_code,
            self.timestamp,
            self.ip_address,
            self.device_id,
            self.channel,
            self.metadata
        );

        let (nested_errors, location) = into_nested_validators!(self.location);

        errors.extend(nested_errors);

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
