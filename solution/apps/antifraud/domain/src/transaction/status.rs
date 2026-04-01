use lib::domain::impl_try_from_string;
use result_like::BoolLike;
use strum::{EnumString, VariantNames};

#[derive(
    BoolLike, EnumString, VariantNames, PartialEq, Eq, Clone, Copy, Debug,
)]
#[strum(serialize_all = "UPPERCASE")]
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
