use std::sync::LazyLock;

use lib::domain::validation::constraints::{self, ConstraintVec};

pub static EMAIL_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(constraints::length::Max(254))
            .add_constraint(constraints::IsValidEmail)
    });

pub static PASSWORD_LENGTH_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(constraints::length::Min(8))
            .add_constraint(constraints::length::Max(72))
    });
