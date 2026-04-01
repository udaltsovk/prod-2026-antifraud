use domain::user::gender::UserGender;
use model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type, Debug)]
#[mapper(ty = UserGender, from, into)]
#[sqlx(type_name = "user_gender", rename_all = "UPPERCASE")]
pub enum StoredUserGender {
    Male,
    Female,
}
