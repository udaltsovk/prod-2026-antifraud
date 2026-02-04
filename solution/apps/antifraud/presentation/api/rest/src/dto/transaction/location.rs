use domain::transaction::location::TransactionLocation;
use lib::{
    model_mapper::Mapper,
    presentation::api::rest::{
        into_validators,
        validation::{
            UserInput, parseable::Parseable, validator::ValidatorResult,
        },
    },
};
use serde::{Deserialize, Serialize};

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = TransactionLocation, from)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLocationDto {
    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,

    #[mapper(opt)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
}

#[derive(Deserialize, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionLocationDto {
    #[serde(default)]
    pub country: UserInput<String>,

    #[serde(default)]
    pub city: UserInput<String>,

    #[serde(default)]
    pub latitude: UserInput<f32>,

    #[serde(default)]
    pub longitude: UserInput<f32>,
}

impl Parseable<TransactionLocation> for CreateTransactionLocationDto {
    fn parse(self) -> ValidatorResult<TransactionLocation> {
        let (mut errors, (country, city)) = into_validators!(
            field!(self.country, optional, "country"),
            field!(self.city, optional, "city")
        );

        let (coords_errors, (latitude, longitude)) = match (
            &self.latitude,
            &self.longitude,
        ) {
            (UserInput::Ok(_), _) | (_, UserInput::Ok(_)) => {
                let (coords_errors, (latitude, longitude)) = into_validators!(
                    field!(self.latitude, required, "latitude"),
                    field!(self.longitude, required, "longitude")
                );

                let latitude = latitude.map(Some);
                let longitude = longitude.map(Some);

                (coords_errors, (latitude, longitude))
            },
            _ => {
                into_validators!(
                    field!(self.latitude, optional, "latitude"),
                    field!(self.longitude, optional, "longitude")
                )
            },
        };

        errors.extend(coords_errors);

        errors.into_result(|ok| TransactionLocation {
            country: country.validated(ok),
            city: city.validated(ok),
            latitude: latitude.validated(ok),
            longitude: longitude.validated(ok),
        })
    }
}
