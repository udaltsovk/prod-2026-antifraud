use lib::domain::Id;

use crate::{
    email::Email,
    session::password::SessionPassword,
    user::{User, role::UserRole},
};

pub mod password;

#[derive(PartialEq, Eq, Debug)]
pub struct Session {
    pub user_id: Id<User>,
    pub user_role: UserRole,
}

impl Session {
    // one hour
    pub const LIFETIME: usize = 60 * 60;
}

#[derive(Debug)]
pub struct CreateSession {
    pub email: Email,
    pub password: SessionPassword,
}
