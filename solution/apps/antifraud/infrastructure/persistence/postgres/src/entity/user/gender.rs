use domain::user::gender::UserGender;
use lib::model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type)]
#[mapper(ty = UserGender, from, into)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[sqlx(type_name = "user_gender", rename_all = "lowercase")]
pub enum StoredUserGender {
    Male,
    Female,
}
