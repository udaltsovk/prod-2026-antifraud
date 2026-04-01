use domain::{session::Session, user::role::UserRole};
use lib::{chrono::Utc, uuid::Uuid};
use model_mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Mapper, Serialize, Deserialize)]
#[mapper(ty = UserRole, from, into)]
#[serde(rename_all = "UPPERCASE")]
pub enum JWTRole {
    Admin,
    User,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    role: JWTRole,
    exp: usize,
    iat: usize,
}

impl From<Session> for Claims {
    fn from(session: Session) -> Self {
        let current_time =
            usize::try_from(Utc::now().timestamp()).unwrap_or(usize::MAX);
        Self {
            sub: session.user_id.value,
            role: session.user_role.into(),
            exp: current_time.saturating_add(Session::LIFETIME),
            iat: current_time,
        }
    }
}

impl From<Claims> for Session {
    fn from(
        Claims {
            sub,
            role,
            ..
        }: Claims,
    ) -> Self {
        Self {
            user_id: sub.into(),
            user_role: role.into(),
        }
    }
}
