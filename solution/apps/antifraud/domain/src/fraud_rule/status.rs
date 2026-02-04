use lib::domain::impl_try_from_external_input;
use result_like::BoolLike;

#[derive(BoolLike, PartialEq, Eq, Clone, Copy, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum FraudRuleStatus {
    #[default]
    Enabled,
    Disabled,
}

impl_try_from_external_input!(domain_type = FraudRuleStatus, input_type = bool);
