use chrono::{DateTime, Utc};
use lib::domain::{
    Id,
    validation::{ExternalInput, Nullable, Optional},
};

use crate::{
    email::Email,
    password::{Password, PasswordHash},
    user::{
        age::UserAge, full_name::UserFullName, gender::UserGender,
        is_active::UserStatus, marital_status::UserMaritalStatus,
        region::UserRegion, role::UserRole,
    },
};

pub mod age;
pub mod full_name;
pub mod gender;
pub mod is_active;
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
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CreateUser {
    pub email: Email,
    pub full_name: UserFullName,
    pub password: Password,
    pub age: Optional<UserAge>,
    pub gender: Optional<UserGender>,
    pub marital_status: Optional<UserMaritalStatus>,
    pub region: Optional<UserRegion>,
    pub role: UserRole,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserCommonUpdate {
    pub full_name: UserFullName,
    pub age: Nullable<UserAge>,
    pub gender: Nullable<UserGender>,
    pub marital_status: Nullable<UserMaritalStatus>,
    pub region: Nullable<UserRegion>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct RawUserAdminUpdate {
    pub status: ExternalInput<bool>,
    pub role: ExternalInput<String>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserUpdate {
    pub common: UserCommonUpdate,
    pub status: Optional<UserStatus>,
    pub role: Optional<UserRole>,
}

impl UserUpdate {
    #[must_use]
    pub fn apply_to(self, user: User) -> User {
        let User {
            id,
            email,
            password_hash,
            role,
            status,
            created_at,
            updated_at,
            ..
        } = user;

        let Self {
            common:
                UserCommonUpdate {
                    full_name: full_name_update,
                    age: age_update,
                    gender: gender_update,
                    marital_status: marital_status_update,
                    region: region_update,
                },
            status: status_update,
            role: role_update,
        } = self;

        User {
            id,
            email,
            full_name: full_name_update,
            password_hash,
            age: age_update.into(),
            gender: gender_update.into(),
            marital_status: marital_status_update.into(),
            region: region_update.into(),
            role: role_update.unwrap_or(role),
            status: status_update.unwrap_or(status),
            created_at,
            updated_at,
        }
    }
}
