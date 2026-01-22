use chrono::{DateTime, Utc};
use lib::domain::Id;

use crate::{
    email::Email,
    password::{Password, PasswordHash},
    user::{
        age::UserAge, full_name::UserFullName, gender::UserGender,
        marital_status::UserMaritalStatus, region::UserRegion, role::UserRole,
    },
};

pub mod age;
pub mod full_name;
pub mod gender;
pub mod marital_status;
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
    pub marital_status: Option<UserMaritalStatus>,
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
    pub marital_status: Option<UserMaritalStatus>,
    pub region: Option<UserRegion>,
    pub role: UserRole,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserUpdate {
    pub full_name: UserFullName,
    pub age: Option<UserAge>,
    pub gender: Option<UserGender>,
    pub marital_status: Option<UserMaritalStatus>,
    pub region: Option<UserRegion>,
    pub is_active: Option<bool>,
    pub role: Option<UserRole>,
}

impl UserUpdate {
    #[must_use]
    pub fn apply_to(self, user: User) -> User {
        let User {
            id,
            email,
            password_hash,
            role,
            is_active,
            created_at,
            updated_at,
            ..
        } = user;

        let Self {
            full_name: full_name_update,
            age: age_update,
            gender: gender_update,
            marital_status: marital_status_update,
            region: region_update,
            is_active: is_active_update,
            role: role_update,
        } = self;

        User {
            id,
            email,
            full_name: full_name_update,
            password_hash,
            age: age_update,
            gender: gender_update,
            marital_status: marital_status_update,
            region: region_update,
            role: role_update.unwrap_or(role),
            is_active: is_active_update.unwrap_or(is_active),
            created_at,
            updated_at,
        }
    }
}
