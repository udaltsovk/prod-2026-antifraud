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

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserRegion(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("region")
        .add_constraint(constraints::length::Max(32))
        .build()
});

impl TryFrom<String> for UserRegion {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = UserRegion,
    input_type = String,
    constraints = CONSTRAINTS
);
