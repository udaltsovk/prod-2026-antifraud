use chrono::{DateTime, Utc};
use lib::domain::Id;

use crate::{
    fraud_rule::result::{FraudRuleResult, status::FraudRuleResultStatus},
    transaction::{
        amount::TransactionAmount, channel::TransactionChannel,
        currency::TransactionCurrency, device_id::TransactionDeviceId,
        ip_address::TransactionIpAddress, location::TransactionLocation,
        merchant_category_code::TransactionMerchantCategoryCode,
        merchant_id::TransactionMerchantId, metadata::TransactionMetadata,
        status::TransactionStatus, timestamp::TransactionTimestamp,
        user_id::TransactionUserId,
    },
};

pub mod amount;
pub mod channel;
pub mod currency;
pub mod decision;
pub mod device_id;
pub mod ip_address;
pub mod location;
pub mod merchant_category_code;
pub mod merchant_id;
pub mod metadata;
pub mod pagination;
pub mod status;
pub mod timestamp;
pub mod user_id;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Transaction {
    pub id: Id<Self>,
    pub user_id: TransactionUserId,
    pub amount: TransactionAmount,
    pub currency: TransactionCurrency,
    pub status: TransactionStatus,
    pub merchant_id: Option<TransactionMerchantId>,
    pub merchant_category_code: Option<TransactionMerchantCategoryCode>,
    pub timestamp: TransactionTimestamp,
    pub ip_address: Option<TransactionIpAddress>,
    pub device_id: Option<TransactionDeviceId>,
    pub channel: Option<TransactionChannel>,
    pub location: TransactionLocation,
    pub metadata: Option<TransactionMetadata>,
    pub created_at: DateTime<Utc>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CreateTransaction {
    pub user_id: TransactionUserId,
    pub amount: TransactionAmount,
    pub currency: TransactionCurrency,
    pub merchant_id: Option<TransactionMerchantId>,
    pub merchant_category_code: Option<TransactionMerchantCategoryCode>,
    pub timestamp: TransactionTimestamp,
    pub ip_address: Option<TransactionIpAddress>,
    pub device_id: Option<TransactionDeviceId>,
    pub channel: Option<TransactionChannel>,
    pub location: TransactionLocation,
    pub metadata: Option<TransactionMetadata>,
}

impl CreateTransaction {
    #[must_use]
    pub fn commit(self, rule_results: &[FraudRuleResult]) -> Transaction {
        let Self {
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
        } = self;

        let status = if rule_results
            .iter()
            .any(|result| result.status == FraudRuleResultStatus::Matched)
        {
            TransactionStatus::Declined
        } else {
            TransactionStatus::Approved
        };

        Transaction {
            id: Id::generate(),
            user_id,
            amount,
            currency,
            status,
            merchant_id,
            merchant_category_code,
            timestamp,
            ip_address,
            device_id,
            channel,
            location,
            metadata,
            created_at: Utc::now(),
        }
    }
}
