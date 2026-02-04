use lib::domain::impl_try_from_string;
use result_like::BoolLike;
use strum::{EnumString, VariantNames};

#[derive(BoolLike, EnumString, VariantNames, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TransactionStatus {
    Approved,
    Declined,
}

impl_try_from_string!(
    enum = TransactionStatus
);

impl TransactionStatus {
    #[must_use]
    pub const fn is_fraudulent(&self) -> bool {
        matches!(self, Self::Declined)
    }
}
