use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        impl_try_from_external_input,
        validation::{
            Constraints, constraints,
            error::{ValidationErrors, ValidationResult},
        },
    },
};

#[derive(DomainType, Clone, Copy, Default, Debug)]
pub struct RuleMatchesStatsFilterTop(i64);

static CONSTRAINTS: LazyLock<Constraints<i64>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(
            constraints::range::Min::with_err(|_, limit| {
                format!("can't be less than {limit}")
            })
            .limit(1_i64)
            .build(),
        )
        .add_constraint(
            constraints::range::Max::with_err(|_, limit| {
                format!("can't be greater than {limit}")
            })
            .limit(100_i64)
            .build(),
        )
        .build()
});

impl TryFrom<i64> for RuleMatchesStatsFilterTop {
    type Error = ValidationErrors;

    fn try_from(value: i64) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = RuleMatchesStatsFilterTop,
    input_type = i64
);

impl From<RuleMatchesStatsFilterTop> for u64 {
    fn from(value: RuleMatchesStatsFilterTop) -> Self {
        value.0.try_into().unwrap_or(Self::MIN)
    }
}
