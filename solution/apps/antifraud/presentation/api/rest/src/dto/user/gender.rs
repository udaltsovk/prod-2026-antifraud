use domain::user::gender::UserGender;
use lib::model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Deserialize, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = UserGender, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserGenderDto {
    Male,
    Female,
}
