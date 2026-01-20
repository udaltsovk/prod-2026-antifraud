use derive_more::From;
use domain::session::CreateSession;
use lib::{
    domain::{into_validators, validation::error::ValidationErrors},
    presentation::api::rest::model::ParseableJson,
};
use serde::{Deserialize, Serialize};

#[derive(From, Serialize, Debug)]
pub struct JsonUserSession {
    token: String,
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
