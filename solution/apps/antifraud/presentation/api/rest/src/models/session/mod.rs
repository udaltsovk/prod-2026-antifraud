use domain::session::CreateSession;
use lib::{
    domain::{into_validators, validation::error::ValidationErrors},
    presentation::api::rest::model::ParseableJson,
};
use serde::{Deserialize, Serialize};

use crate::models::user::JsonUser;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonUserSession {
    access_token: String,

    expires_in: usize,

    user: JsonUser,
}

impl From<(String, JsonUser)> for JsonUserSession {
    fn from((access_token, user): (String, JsonUser)) -> Self {
        Self {
            access_token,
            expires_in: 3600,
            user,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateJsonSession {
    email: String,
    password: String,
}

impl ParseableJson<CreateSession> for CreateJsonSession {
    fn parse(self) -> Result<CreateSession, ValidationErrors> {
        let (errors, (email, password)) =
            into_validators!(self.email, self.password);

        errors.into_result(|ok| CreateSession {
            email: email.validated(ok),
            password: password.validated(ok),
        })
    }
}
