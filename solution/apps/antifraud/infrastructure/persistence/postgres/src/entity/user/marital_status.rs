use domain::user::marital_status::UserMaritalStatus;
use lib::model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type)]
#[mapper(ty = UserMaritalStatus, from, into)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[sqlx(type_name = "user_marital_status", rename_all = "lowercase")]
pub enum StoredUserMaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}
