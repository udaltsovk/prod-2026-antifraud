use chrono::{DateTime, Utc};
use domain::user::{CreateUser, User, role::UserRole};
use lib::{
    domain::{
        into_option_validators, into_validators,
        validation::error::ValidationResult,
    },
    model_mapper::Mapper,
    presentation::api::rest::{into_nested_validators, model::Parseable},
    uuid::Uuid,
};
use serde::{Deserialize, Serialize};

use crate::models::user::{
    gender::JsonUserGender, marital_status::JsonUserMaritalStatus,
    role::JsonUserRole,
};

pub mod gender;
pub mod marital_status;
pub mod role;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = User, from, ignore_extra)]
#[serde(rename_all = "camelCase")]
pub struct JsonUser {
    pub id: Uuid,

    pub email: String,

    pub full_name: String,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u8>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<JsonUserGender>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<JsonUserMaritalStatus>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    pub role: JsonUserRole,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateJsonUser {
    pub email: Option<String>,

    pub full_name: Option<String>,

    pub password: Option<String>,

    pub age: Option<i64>,

    pub gender: Option<String>,

    pub marital_status: Option<String>,

    pub region: Option<String>,
}

impl Parseable<CreateUser> for CreateJsonUser {
    const FIELD: &str = "user";

    fn parse(self) -> ValidationResult<CreateUser> {
        let (mut errors, (email, full_name, password)) =
            into_validators!(self.email, self.full_name, self.password);

        let (option_errors, (age, gender, marital_status, region)) = into_option_validators!(
            self.age,
            self.gender,
            self.marital_status,
            self.region
        );

        errors.extend(option_errors);

        errors.into_result(|ok| CreateUser {
            email: email.validated(ok),
            full_name: full_name.validated(ok),
            password: password.validated(ok),
            role: UserRole::User,
            age: age.validated(ok),
            gender: gender.validated(ok),
            marital_status: marital_status.validated(ok),
            region: region.validated(ok),
        })
    }
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateJsonUserWithRole {
    #[serde(flatten)]
    pub inner: CreateJsonUser,

    pub role: String,
}

impl Parseable<CreateUser> for CreateJsonUserWithRole {
    const FIELD: &str = "user";

    fn parse(self) -> ValidationResult<CreateUser> {
        let (mut errors, role) = into_validators!(self.role);
        let (nested_errors, inner) = into_nested_validators!(self.inner);

        errors.extend(nested_errors);

        errors.into_result(|ok| CreateUser {
            role: role.validated(ok),
            ..inner.validated(ok)
        })
    }
}
