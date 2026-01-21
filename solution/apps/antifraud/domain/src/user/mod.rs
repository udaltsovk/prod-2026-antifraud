use chrono::{DateTime, Utc};
use lib::domain::Id;

use crate::{
    email::Email,
    password::{Password, PasswordHash},
    user::{
        age::UserAge, full_name::UserFullName, gender::UserGender,
        martial_status::UserMartialStatus, region::UserRegion, role::UserRole,
    },
};

pub mod age;
pub mod full_name;
pub mod gender;
pub mod martial_status;
pub mod region;
pub mod role;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct User {
    pub id: Id<Self>,
    pub email: Email,
    pub full_name: UserFullName,
    pub password_hash: PasswordHash,
    pub age: Option<UserAge>,
    pub gender: Option<UserGender>,
    pub martial_status: Option<UserMartialStatus>,
    pub region: Option<UserRegion>,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CreateUser {
    pub email: Email,
    pub full_name: UserFullName,
    pub password: Password,
    pub age: Option<UserAge>,
    pub gender: Option<UserGender>,
    pub martial_status: Option<UserMartialStatus>,
    pub region: Option<UserRegion>,
    pub role: UserRole,
}
