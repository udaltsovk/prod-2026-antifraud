use domain::session::CreateSession;
use lib::{
    presentation::api::rest::{
        into_validators,
        validation::{
            UserInput, parseable::Parseable, validator::ValidatorResult,
        },
    },
    redact::{Secret, expose_secret},
};
use serde::{Deserialize, Serialize};

use crate::dto::user::UserDto;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserSessionDto {
    #[serde(serialize_with = "expose_secret")]
    access_token: Secret<String>,

    expires_in: usize,

    user: UserDto,
}

impl From<(Secret<String>, UserDto)> for UserSessionDto {
    fn from((access_token, user): (Secret<String>, UserDto)) -> Self {
        Self {
            access_token,
            expires_in: 3600,
            user,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateSessionDto {
    email: UserInput<String>,

    password: UserInput<Secret<String>>,
}

impl Parseable<CreateSession> for CreateSessionDto {
    fn parse(self) -> ValidatorResult<CreateSession> {
        let (errors, (email, password)) = into_validators!(
            field!(self.email, required, "email"),
            field!(self.password, required, "password")
        );

        errors.into_result(|ok| CreateSession {
            email: email.validated(ok),
            password: password.validated(ok),
        })
    }
}
