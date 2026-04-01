use lib::domain::impl_try_from_external_input;
use result_like::BoolLike;

#[derive(BoolLike, PartialEq, Eq, Clone, Copy, Debug)]
pub enum FraudRuleResultStatus {
    Matched,
    Unmatched,
}

impl_try_from_external_input!(
    domain_type = FraudRuleResultStatus,
    input_type = bool
);
