use domain::user::martial_status::UserMartialStatus;
use lib::model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = UserMartialStatus, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum JsonUserMartialStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}
