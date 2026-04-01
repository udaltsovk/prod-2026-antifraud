use domain::transaction::location::TransactionLocation;
use lib::{
    domain::DomainType, infrastructure::persistence::entity::DomainTypeFromDb,
};
use model_mapper::Mapper;
use sqlx::Type;

#[derive(Mapper, Type, Debug)]
#[mapper(ty = TransactionLocation, from, into)]
#[sqlx(type_name = "transaction_location")]
pub struct StoredTransactionLocation {
    #[mapper(
        opt(
            from_with = DomainType::into_inner,
            into_with = DomainTypeFromDb::into_domain
        )
    )]
    pub country: Option<String>,

    #[mapper(
        opt(
            from_with = DomainType::into_inner,
            into_with = DomainTypeFromDb::into_domain
        )
    )]
    pub city: Option<String>,

    #[mapper(
        opt(
            from_with = DomainType::into_inner,
            into_with = DomainTypeFromDb::into_domain
        )
    )]
    pub latitude: Option<f32>,

    #[mapper(
        opt(
            from_with = DomainType::into_inner,
            into_with = DomainTypeFromDb::into_domain
        )
    )]
    pub longitude: Option<f32>,
}
