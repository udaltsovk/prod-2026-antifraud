use lib::domain::impl_try_from_external_input;
use result_like::BoolLike;

#[derive(BoolLike, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum UserStatus {
    Active,
    Deactivated,
}

impl_try_from_external_input!(domain_type = UserStatus, input_type = bool);
