use domain::user::marital_status::UserMaritalStatus;
use lib::model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = UserMaritalStatus, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum JsonUserMaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}
