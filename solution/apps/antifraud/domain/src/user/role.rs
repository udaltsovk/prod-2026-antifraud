use lib::domain::impl_try_from_string;
use strum::{EnumString, VariantNames};

#[derive(EnumString, VariantNames, Clone, Copy, PartialEq, Eq, Debug)]
#[strum(serialize_all = "UPPERCASE")]
pub enum UserRole {
    Admin,
    User,
}

impl_try_from_string!(
    enum = UserRole
);
