use domain::session::CreateSession;
use lib::{
    domain::{into_validators, validation::error::ValidationResult},
    presentation::api::rest::model::Parseable,
};
use serde::{Deserialize, Serialize};

use crate::models::user::JsonUser;

#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
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

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CreateJsonSession {
    email: String,
    password: String,
}

impl Parseable<CreateSession> for CreateJsonSession {
    const FIELD: &str = "credentials";

    fn parse(self) -> ValidationResult<CreateSession> {
        let (errors, (email, password)) =
            into_validators!(self.email, self.password);

        errors.into_result(|ok| CreateSession {
            email: email.validated(ok),
            password: password.validated(ok),
        })
    }
}
