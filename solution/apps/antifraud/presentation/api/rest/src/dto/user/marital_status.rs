use domain::user::marital_status::UserMaritalStatus;
use model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize, Debug)]
#[mapper(ty = UserMaritalStatus, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserMaritalStatusDto {
    Single,
    Married,
    Divorced,
    Widowed,
}
