use chrono::{DateTime, Utc};
use lib::domain::Id;

use crate::{
    email::Email,
    password::PasswordHash,
    user::{
        age::UserAge, full_name::UserFullName, gender::UserGender,
        marital_status::UserMaritalStatus, password::UserPassword,
        region::UserRegion, role::UserRole, status::UserStatus,
    },
};

pub mod age;
pub mod filter;
pub mod full_name;
pub mod gender;
pub mod marital_status;
pub mod password;
pub mod region;
pub mod role;
pub mod status;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CreateUser {
    pub email: Email,
    pub full_name: UserFullName,
    pub password: UserPassword,
    pub age: Option<UserAge>,
    pub gender: Option<UserGender>,
    pub marital_status: Option<UserMaritalStatus>,
    pub region: Option<UserRegion>,
    pub role: UserRole,
}

#[derive(Debug)]
pub struct UserUpdate {
    pub full_name: UserFullName,
    pub age: Option<UserAge>,
    pub gender: Option<UserGender>,
    pub marital_status: Option<UserMaritalStatus>,
    pub region: Option<UserRegion>,
    pub status: Option<UserStatus>,
    pub role: Option<UserRole>,
}

impl PartialEq<User> for UserUpdate {
    fn eq(&self, other: &User) -> bool {
        let User {
            full_name: current_full_name,
            age: current_age,
            gender: current_gender,
            marital_status: current_marital_status,
            region: current_region,
            role: current_role,
            status: current_status,
            ..
        } = other;

        let Self {
            full_name: new_full_name,
            age: new_age,
            gender: new_gender,
            marital_status: new_marital_status,
            region: new_region,
            status: new_status,
            role: new_role,
        } = self;

        new_full_name == current_full_name
            && new_age == current_age
            && new_gender == current_gender
            && new_marital_status == current_marital_status
            && new_region == current_region
            && new_role.as_ref() == Some(current_role)
            && new_status.as_ref() == Some(current_status)
    }
}

impl UserUpdate {
    #[must_use]
    pub fn apply_to(self, user: User) -> User {
        let User {
            id: current_id,
            email: current_email,
            password_hash: current_password_hash,
            role: current_role,
            status: current_status,
            created_at: current_created_at,
            updated_at: current_updated_at,
            ..
        } = user;

        let Self {
            full_name: full_name_update,
            age: age_update,
            gender: gender_update,
            marital_status: marital_status_update,
            region: region_update,
            status: status_update,
            role: role_update,
        } = self;

        User {
            id: current_id,
            email: current_email,
            full_name: full_name_update,
            password_hash: current_password_hash,
            age: age_update,
            gender: gender_update,
            marital_status: marital_status_update,
            region: region_update,
            role: role_update.unwrap_or(current_role),
            status: status_update.unwrap_or(current_status),
            created_at: current_created_at,
            updated_at: current_updated_at,
        }
    }
}

pub struct UserActivity;
