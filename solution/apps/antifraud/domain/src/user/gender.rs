use lib::domain::try_from_string;
use strum::{EnumString, VariantNames};

#[derive(EnumString, VariantNames)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[strum(serialize_all = "UPPERCASE")]
pub enum UserGender {
    Male,
    Female,
}

try_from_string!(
    enum = UserGender,
    field = "gender"
);
