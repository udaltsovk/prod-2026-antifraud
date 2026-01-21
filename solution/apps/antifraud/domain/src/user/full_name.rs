use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, constraints, error::ValidationErrors},
};

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserFullName(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("full_name")
        .add_constraint(constraints::length::Min(2))
        .add_constraint(constraints::length::Max(200))
        .build()
});

impl TryFrom<String> for UserFullName {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
