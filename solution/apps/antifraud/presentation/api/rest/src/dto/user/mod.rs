use chrono::{DateTime, Utc};
use domain::user::{CreateUser, User, UserUpdate, role::UserRole};
use lib::{
    model_mapper::Mapper,
    presentation::api::rest::{
        into_validators,
        validation::{
            UserInput, parseable::Parseable, validator::ValidatorResult,
        },
    },
    redact::Secret,
    uuid::Uuid,
};
use serde::{Deserialize, Serialize};

use crate::dto::user::{
    gender::UserGenderDto, marital_status::UserMaritalStatusDto,
    role::UserRoleDto,
};

pub mod filter;
pub mod gender;
pub mod marital_status;
pub mod role;

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = User, from, ignore_extra)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,

    pub email: String,

    pub full_name: String,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u8>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<UserGenderDto>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<UserMaritalStatusDto>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    pub role: UserRoleDto,

    #[mapper(rename = status)]
    pub is_active: bool,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateUserDto {
    #[serde(default)]
    pub email: UserInput<String>,

    #[serde(default)]
    pub full_name: UserInput<String>,

    #[serde(default)]
    pub password: UserInput<Secret<String>>,

    #[serde(default)]
    pub age: UserInput<i64>,

    #[serde(default)]
    pub gender: UserInput<String>,

    #[serde(default)]
    pub marital_status: UserInput<String>,

    #[serde(default)]
    pub region: UserInput<String>,
}

impl Parseable<CreateUser> for CreateUserDto {
    fn parse(self) -> ValidatorResult<CreateUser> {
        let (
            errors,
            (email, full_name, password, age, gender, marital_status, region),
        ) = into_validators!(
            field!(self.email, required, "email"),
            field!(self.full_name, required, "fullName"),
            field!(self.password, required, "password"),
            field!(self.age, optional, "age"),
            field!(self.gender, optional, "gender"),
            field!(self.marital_status, optional, "maritalStatus"),
            field!(self.region, optional, "region")
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
pub struct CreateUserWithRoleDto {
    #[serde(flatten)]
    pub inner: CreateUserDto,

    #[serde(default)]
    pub role: UserInput<String>,
}

impl Parseable<CreateUser> for CreateUserWithRoleDto {
    fn parse(self) -> ValidatorResult<CreateUser> {
        let (errors, (role, inner)) = into_validators!(
            field!(self.role, required, "role"),
            field!(self.inner, nested, None)
        );
        errors.into_result(|ok| CreateUser {
            role: role.validated(ok),
            ..inner.validated(ok)
        })
    }
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateDto {
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

    #[serde(default)]
    pub is_active: UserInput<bool>,

    #[serde(default)]
    pub role: UserInput<String>,
}

impl Parseable<UserUpdate> for UserUpdateDto {
    fn parse(self) -> ValidatorResult<UserUpdate> {
        let (
            errors,
            (age, full_name, gender, marital_status, region, status, role),
        ) = into_validators!(
            field!(self.age, required_nullable, "age"),
            field!(self.full_name, required, "fullName"),
            field!(self.gender, required_nullable, "gender"),
            field!(self.marital_status, required_nullable, "maritalStatus"),
            field!(self.region, required_nullable, "region"),
            field!(self.is_active, optional, "isActive"),
            field!(self.role, optional, "role")
        );

        errors.into_result(|ok| UserUpdate {
            full_name: full_name.validated(ok),
            age: age.validated(ok),
            gender: gender.validated(ok),
            marital_status: marital_status.validated(ok),
            region: region.validated(ok),
            status: status.validated(ok),
            role: role.validated(ok),
        })
    }
}
