use domain::user::role::UserRole;
use lib::model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type)]
#[mapper(ty = UserRole, from, into)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum StoredUserRole {
    Admin,
    User,
}
