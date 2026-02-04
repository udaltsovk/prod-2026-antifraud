use std::sync::Arc;

use domain::{transaction::CreateTransaction, user::User};
use dsl::Context;
use lib::tap::Pipe as _;

pub struct DslServiceContext<'src>(pub Context<'src>);

#[derive(Default)]
#[must_use]
pub struct DslServiceContextBuilder<'src> {
    amount: Option<f64>,
    currency: Option<&'src str>,
    merchant_id: Option<&'src str>,
    ip_address: Option<String>,
    device_id: Option<&'src str>,
    user_age: Option<f64>,
    user_region: Option<&'src str>,
}

impl<'src> DslServiceContextBuilder<'src> {
    const fn with_amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    const fn with_currency(mut self, currency: &'src str) -> Self {
        self.currency = Some(currency);
        self
    }

    const fn with_merchant_id(
        mut self,
        merchant_id: Option<&'src str>,
    ) -> Self {
        self.merchant_id = merchant_id;
        self
    }

    fn with_ip_address(mut self, ip_address: Option<String>) -> Self {
        self.ip_address = ip_address;
        self
    }

    const fn with_device_id(mut self, device_id: Option<&'src str>) -> Self {
        self.device_id = device_id;
        self
    }

    const fn with_user_age(mut self, user_age: Option<f64>) -> Self {
        self.user_age = user_age;
        self
    }

    const fn with_user_region(
        mut self,
        user_region: Option<&'src str>,
    ) -> Self {
        self.user_region = user_region;
        self
    }

    fn build(self) -> DslServiceContext<'src> {
        Context::builder()
            .add_field("amount", self.amount)
            .add_field("currency", self.currency)
            .add_field("merchantId", self.merchant_id)
            .add_field("ipAddress", self.ip_address)
            .add_field("deviceId", self.device_id)
            .add_field("user.age", self.user_age)
            .add_field("user.region", self.user_region)
            .build()
            .pipe(DslServiceContext)
    }
}

impl DslServiceContext<'_> {
    pub fn dummy() -> Self {
        DslServiceContextBuilder::default().build()
    }
}

impl<'src> From<(&'src CreateTransaction, &'src Arc<User>)>
    for DslServiceContext<'src>
{
    fn from(
        (transaction, user): (&'src CreateTransaction, &'src Arc<User>),
    ) -> Self {
        DslServiceContextBuilder::default()
            .with_amount(*transaction.amount.as_ref())
            .with_currency(transaction.currency.as_ref())
            .with_merchant_id(
                transaction
                    .merchant_id
                    .as_ref()
                    .map(AsRef::as_ref)
                    .map(String::as_str),
            )
            .with_ip_address(
                transaction
                    .merchant_id
                    .as_ref()
                    .map(|ip| ip.as_ref().clone()),
            )
            .with_device_id(
                transaction
                    .device_id
                    .as_ref()
                    .map(AsRef::as_ref)
                    .map(String::as_str),
            )
            .with_user_age(
                user.age.as_ref().map(|age| f64::from(*age.as_ref())),
            )
            .with_user_region(
                user.region.as_ref().map(AsRef::as_ref).map(String::as_str),
            )
            .build()
    }
}
