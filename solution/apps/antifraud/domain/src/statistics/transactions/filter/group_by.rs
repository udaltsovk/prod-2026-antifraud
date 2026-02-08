use lib::domain::impl_try_from_string;
use strum::{Display, EnumString, VariantNames};

#[derive(
    EnumString, VariantNames, Display, Clone, Copy, PartialEq, Eq, Default,
)]
#[cfg_attr(debug_assertions, derive(Debug))]
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
