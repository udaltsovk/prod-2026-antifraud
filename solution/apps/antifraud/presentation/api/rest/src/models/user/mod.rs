use chrono::{DateTime, Utc};
use domain::user::{
    CreateUser, RawUserAdminUpdate, User, UserCommonUpdate, role::UserRole,
};
use lib::{
    domain::{into_validators, validation::error::ValidationResult},
    model_mapper::Mapper,
    presentation::api::rest::{
        UserInput, into_nested_validators, model::Parseable,
    },
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

    #[mapper(rename = status)]
    pub is_active: bool,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateJsonUser {
    #[serde(default)]
    pub email: UserInput<String>,

    #[serde(default)]
    pub full_name: UserInput<String>,

    #[serde(default)]
    pub password: UserInput<String>,

    #[serde(default)]
    pub age: UserInput<i64>,

    #[serde(default)]
    pub gender: UserInput<String>,

    #[serde(default)]
    pub marital_status: UserInput<String>,

    #[serde(default)]
    pub region: UserInput<String>,
}

impl Parseable<CreateUser> for CreateJsonUser {
    const FIELD: &str = "user";

    fn parse(self) -> ValidationResult<CreateUser> {
        let (
            errors,
            (email, full_name, password, age, gender, marital_status, region),
        ) = into_validators!(
            self.email,
            self.full_name,
            self.password,
            self.age,
            self.gender,
            self.marital_status,
            self.region
        );

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

    #[serde(default)]
    pub role: UserInput<String>,
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

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct JsonUserCommonUpdate {
    #[serde(default)]
    pub age: UserInput<i64>,

    #[serde(default)]
    pub full_name: UserInput<String>,

    #[serde(default)]
    pub gender: UserInput<String>,

    #[serde(default)]
    pub marital_status: UserInput<String>,

    #[serde(default)]
    pub region: UserInput<String>,
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct JsonUserUpdate {
    #[serde(flatten)]
    pub common: JsonUserCommonUpdate,

    #[serde(default)]
    pub is_active: UserInput<bool>,

    #[serde(default)]
    pub role: UserInput<String>,
}

impl From<JsonUserUpdate> for (JsonUserCommonUpdate, RawUserAdminUpdate) {
    fn from(
        JsonUserUpdate {
            common: common_update,
            is_active: status,
            role,
        }: JsonUserUpdate,
    ) -> Self {
        (
            common_update,
            RawUserAdminUpdate {
                status: status.into(),
                role: role.into(),
            },
        )
    }
}

impl Parseable<UserCommonUpdate> for JsonUserCommonUpdate {
    const FIELD: &str = "user";

    fn parse(self) -> ValidationResult<UserCommonUpdate> {
        let (errors, (age, full_name, gender, marital_status, region)) = into_validators!(
            self.age,
            self.full_name,
            self.gender,
            self.marital_status,
            self.region
        );

        errors.into_result(|ok| UserCommonUpdate {
            full_name: full_name.validated(ok),
            age: age.validated(ok),
            gender: gender.validated(ok),
            marital_status: marital_status.validated(ok),
            region: region.validated(ok),
        })
    }
}
