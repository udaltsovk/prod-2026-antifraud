use lib::domain::impl_try_from_string;
use strum::{Display, EnumString, VariantNames};

#[derive(
    EnumString,
    VariantNames,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Debug,
)]
#[strum(serialize_all = "lowercase")]
pub enum TransactionsTimeseriesPointFilterGroupBy {
    Hour,
    #[default]
    Day,
    Week,
}

impl_try_from_string!(
    enum = TransactionsTimeseriesPointFilterGroupBy
);
