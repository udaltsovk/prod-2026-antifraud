use domain::user::role::UserRole;
use lib::model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = UserRole, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserRoleDto {
    Admin,
    User,
}
