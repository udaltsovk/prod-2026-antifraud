use std::sync::LazyLock;

use lib::domain::validation::constraints::{self, ConstraintVec};

pub static EMAIL_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(
                constraints::length::Max::with_err(|_, len_limit| {
                    format!("can't be longer than {len_limit} characters")
                })
                .limit(254)
                .build(),
            )
            .add_constraint(
                constraints::IsValidEmail::with_err(|_| {
                    "must be a valid email".to_string()
                })
                .build(),
            )
    });

pub static PASSWORD_LENGTH_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(
                constraints::length::Min::with_err(|_, len_limit| {
                    format!("can't be shorter than {len_limit} characters")
                })
                .limit(8)
                .build(),
            )
            .add_constraint(
                constraints::length::Max::with_err(|_, len_limit| {
                    format!("can't be longer than {len_limit} characters")
                })
                .limit(72)
                .build(),
            )
    });
