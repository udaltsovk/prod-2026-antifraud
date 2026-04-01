use domain::user::role::UserRole;
use model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize, Debug)]
#[mapper(ty = UserRole, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserRoleDto {
    Admin,
    User,
}
