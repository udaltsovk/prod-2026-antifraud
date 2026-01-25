use domain::transaction::location::TransactionLocation;
use lib::{
    domain::DomainType, infrastructure::persistence::entity::DomainTypeFromDb,
    model_mapper::Mapper,
};
use sqlx::Type;

#[derive(Mapper, Type)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = TransactionLocation, from, into)]
#[sqlx(type_name = "transaction_location")]
pub struct StoredTransactionLocation {
    #[mapper(
        from_with = country.map(DomainType::into_inner).into_option(),
        into_with = country.map(DomainTypeFromDb::into_domain).into()
    )]
    pub country: Option<String>,

    #[mapper(
        from_with = city.map(DomainType::into_inner).into_option(),
        into_with = city.map(DomainTypeFromDb::into_domain).into()
    )]
    pub city: Option<String>,

    #[mapper(
        from_with = latitude.map(DomainType::into_inner).into_option(),
        into_with = latitude.map(DomainTypeFromDb::into_domain).into()
    )]
    pub latitude: Option<f32>,

    #[mapper(
        from_with = longitude.map(DomainType::into_inner).into_option(),
        into_with = longitude.map(DomainTypeFromDb::into_domain).into()
    )]
    pub longitude: Option<f32>,
}
