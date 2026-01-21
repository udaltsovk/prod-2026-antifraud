use chrono::{DateTime, Utc};
use domain::user::{
    CreateUser, User, gender::UserGender, martial_status::UserMartialStatus,
    role::UserRole,
};
use lib::{
    domain::{
        into_option_validators, into_validators,
        validation::error::ValidationErrors,
    },
    model_mapper::Mapper,
    presentation::api::rest::model::ParseableJson,
    uuid::Uuid,
};
use serde::{Deserialize, Serialize};

use crate::models::user::{
    gender::JsonUserGender, martial_status::JsonUserMartialStatus,
    role::JsonUserRole,
};

pub mod gender;
pub mod martial_status;
pub mod role;

#[derive(Mapper, Serialize, Debug)]
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
    pub martial_status: Option<JsonUserMartialStatus>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    pub role: JsonUserRole,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateJsonUser {
    pub email: String,

    pub full_name: String,

    pub password: String,

    pub age: Option<i64>,

    pub gender: Option<JsonUserGender>,

    pub martial_status: Option<JsonUserMartialStatus>,

    pub region: Option<String>,
}

impl ParseableJson<CreateUser> for CreateJsonUser {
    fn parse(self) -> Result<CreateUser, ValidationErrors> {
        let (mut errors, (email, full_name, password)) =
            into_validators!(self.email, self.full_name, self.password);

        let (option_errors, (age, region)) =
            into_option_validators!(self.age, self.region);

        errors.extend(option_errors);

        errors.into_result(|ok| CreateUser {
            email: email.validated(ok),
            full_name: full_name.validated(ok),
            password: password.validated(ok),
            age: age.validated(ok),
            gender: self.gender.map(UserGender::from),
            martial_status: self.martial_status.map(UserMartialStatus::from),
            region: region.validated(ok),
            role: UserRole::User,
        })
    }
}
