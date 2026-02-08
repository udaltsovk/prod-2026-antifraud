use lib::domain::impl_try_from_string;
use strum::{EnumString, VariantNames};

#[derive(EnumString, VariantNames, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[strum(serialize_all = "UPPERCASE")]
pub enum TransactionChannel {
    Web,
    Mobile,
    Pos,
    Other,
}

impl_try_from_string!(
    enum = TransactionChannel
);
