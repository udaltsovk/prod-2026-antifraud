use domain::user::marital_status::UserMaritalStatus;
use model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type, Debug)]
#[mapper(ty = UserMaritalStatus, from, into)]
#[sqlx(type_name = "user_marital_status", rename_all = "UPPERCASE")]
pub enum StoredUserMaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}
