use domain::user::gender::UserGender;
use model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize, Debug)]
#[mapper(ty = UserGender, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserGenderDto {
    Male,
    Female,
}
