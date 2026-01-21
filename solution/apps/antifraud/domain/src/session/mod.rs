use lib::domain::Id;

use crate::{
    email::Email,
    password::Password,
    user::{User, role::UserRole},
};

#[derive(PartialEq, Eq)]
pub struct Session {
    pub user_id: Id<User>,
    pub user_role: UserRole,
}

impl Session {
    // one hour
    pub const LIFETIME: usize = 60 * 60;
}

pub struct CreateSession {
    pub email: Email,
    pub password: Password,
}
