use chrono::{DateTime, Utc};
use domain::user::User;
use lib::{
    infrastructure::persistence::entity::DomainTypeFromDb,
    model_mapper::Mapper, uuid::Uuid,
};
use sqlx::FromRow;

use crate::entity::user::{
    gender::StoredUserGender, marital_status::StoredUserMaritalStatus,
    role::StoredUserRole,
};

pub mod gender;
pub mod marital_status;
pub mod role;

#[derive(Mapper, FromRow)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(derive(ty = User, into))]
pub struct StoredUser {
    pub id: Uuid,

    #[mapper(
        when(ty = User, into_with = DomainTypeFromDb::into_domain),
    )]
    pub email: String,

    #[mapper(
        when(ty = User, into_with = DomainTypeFromDb::into_domain),
    )]
    pub full_name: String,

    pub password_hash: String,

    #[mapper(
        when(ty = User, opt(into_with = DomainTypeFromDb::into_domain)),
    )]
    pub age: Option<i16>,

    #[mapper(opt)]
    pub gender: Option<StoredUserGender>,

    #[mapper(opt)]
    pub marital_status: Option<StoredUserMaritalStatus>,

    #[mapper(
        when(ty = User, opt(into_with = DomainTypeFromDb::into_domain)),
    )]
    pub region: Option<String>,

    pub role: StoredUserRole,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}
