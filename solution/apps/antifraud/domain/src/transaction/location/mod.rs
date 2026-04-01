use crate::transaction::location::{
    city::TransactionLocationCity, country::TransactionLocationCountry,
    latitude::TransactionLocationLatitude,
    longitude::TransactionLocationLongitude,
};

pub mod city;
pub mod country;
pub mod latitude;
pub mod longitude;

#[derive(Debug)]
pub struct TransactionLocation {
    pub country: Option<TransactionLocationCountry>,
    pub city: Option<TransactionLocationCity>,
    pub latitude: Option<TransactionLocationLatitude>,
    pub longitude: Option<TransactionLocationLongitude>,
}
