use lib::domain::validation::Optional;

use crate::transaction::location::{
    city::TransactionLocationCity, country::TransactionLocationCountry,
    latitude::TransactionLocationLatitude,
    longitude::TransactionLocationLongitude,
};

pub mod city;
pub mod country;
pub mod latitude;
pub mod longitude;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TransactionLocation {
    pub country: Optional<TransactionLocationCountry>,
    pub city: Optional<TransactionLocationCity>,
    pub latitude: Optional<TransactionLocationLatitude>,
    pub longitude: Optional<TransactionLocationLongitude>,
}
