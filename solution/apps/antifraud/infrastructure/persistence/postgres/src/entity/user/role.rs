use domain::user::role::UserRole;
use model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type, Debug)]
#[mapper(ty = UserRole, from, into)]
#[sqlx(type_name = "user_role", rename_all = "UPPERCASE")]
pub enum StoredUserRole {
    Admin,
    User,
}
