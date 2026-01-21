use std::{
    fmt::{Debug, Display},
    sync::LazyLock,
};

use lib::{
    DomainType,
    domain::{
        DomainType as _,
        validation::{
            Constraints,
            constraints::{self, range::Num},
            error::ValidationErrors,
        },
    },
    paste,
};
use serde::Serialize;

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserAge(u8);

impl UserAge {
    fn constraints<T>() -> Constraints<T>
    where
        T: Num
            + Serialize
            + Clone
            + Debug
            + PartialOrd
            + Display
            + Send
            + Sync
            + 'static,
        T::FromStrRadixErr: Debug,
    {
        Constraints::builder("age")
            .add_constraint(constraints::range::Min(T::zero()))
            .add_constraint(constraints::range::Max(
                T::from_str_radix("120", 10).expect("a valid number"),
            ))
            .build()
    }
}

macro_rules! numeric_constraints {
    ($type: ty) => {
        paste! {
            static [<CONSTRAINTS_ $type:upper>]: LazyLock<Constraints<$type>> =
                LazyLock::new(UserAge::constraints);

            impl TryFrom<$type> for UserAge {
                type Error = ValidationErrors;

                fn try_from(value: $type) -> Result<Self, ValidationErrors> {
                    [<CONSTRAINTS_ $type:upper>].check(&value).into_result(|_| {
                        Self(value.try_into().unwrap_or_else(
                            Self::it_should_be_safe_to_unwrap([<CONSTRAINTS_ $type:upper>].name()),
                        ))
                    })
                }
            }
        }

        impl From<UserAge> for $type {
            fn from(age: UserAge) -> Self {
                age.0.into()
            }
        }
    };
}

numeric_constraints!(i16);
numeric_constraints!(i64);
