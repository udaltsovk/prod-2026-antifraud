use lib::domain::impl_try_from_string;
use strum::{EnumString, VariantNames};

#[derive(EnumString, VariantNames)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[strum(serialize_all = "UPPERCASE")]
pub enum UserMaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}

impl_try_from_string!(
    enum = UserMaritalStatus,
    field = "maritalStatus"
);
