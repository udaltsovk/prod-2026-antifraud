use domain::transaction::location::TransactionLocation;
use lib::{
    domain::{
        into_validators,
        validation::{Optional, error::ValidationResult},
    },
    model_mapper::Mapper,
    presentation::api::rest::{UserInput, model::Parseable},
};
use serde::{Deserialize, Serialize};

#[derive(Mapper, Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = TransactionLocation, from)]
#[serde(rename_all = "camelCase")]
pub struct JsonTransactionLocation {
    #[mapper(with = country.into_option().map(From::from))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[mapper(with = city.into_option().map(From::from))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[mapper(with = latitude.into_option().map(From::from))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,

    #[mapper(with = longitude.into_option().map(From::from))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,
}

#[derive(Deserialize, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CreateJsonTransactionLocation {
    #[serde(default)]
    pub country: UserInput<String>,

    #[serde(default)]
    pub city: UserInput<String>,

    #[serde(default)]
    pub latitude: UserInput<f32>,

    #[serde(default)]
    pub longitude: UserInput<f32>,
}

impl Parseable<TransactionLocation> for CreateJsonTransactionLocation {
    const FIELD: &str = "location";

    fn parse(self) -> ValidationResult<TransactionLocation> {
        let (mut errors, (country, city)) =
            into_validators!(self.country, self.city);

        let (coords_errors, (latitude, longitude)) =
            match (&self.latitude, &self.longitude) {
                (UserInput::Ok(_), _) | (_, UserInput::Ok(_)) => {
                    let (coords_errors, (latitude, longitude)) =
                        into_validators!(self.latitude, self.longitude);

                    let latitude = latitude.map(Optional::Present);
                    let longitude = longitude.map(Optional::Present);

                    (coords_errors, (latitude, longitude))
                },
                _ => {
                    into_validators!(self.latitude, self.longitude)
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
