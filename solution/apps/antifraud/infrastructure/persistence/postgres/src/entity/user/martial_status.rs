use domain::user::martial_status::UserMartialStatus;
use lib::model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type)]
#[mapper(ty = UserMartialStatus, from, into)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[sqlx(type_name = "user_martial_status", rename_all = "lowercase")]
pub enum StoredUserMartialStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}
