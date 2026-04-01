use lib::domain::impl_try_from_string;
use strum::{EnumString, VariantNames};

#[derive(EnumString, VariantNames, PartialEq, Eq, Debug)]
#[strum(serialize_all = "UPPERCASE")]
pub enum UserMaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}

impl_try_from_string!(
    enum = UserMaritalStatus
);
