use application::usecase::{
    session::SessionUseCase as _, user::UserUseCase as _,
};
use axum::{
    RequestPartsExt as _, extract::FromRequestParts, http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use domain::{
    session::Session,
    user::{User, role::UserRole},
};
use lib::{domain::Id, model_mapper::Mapper, redact::Secret, tap::Conv as _};

use crate::{ApiError, ModulesExt, errors::AuthError};

#[derive(Mapper)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[mapper(ty = Session, from)]
pub struct AdminSession {
    pub user_id: Id<User>,
    pub user_role: UserRole,
}

impl From<AdminSession> for (Id<User>, UserRole) {
    fn from(session: AdminSession) -> Self {
        (session.user_id, session.user_role)
    }
}

impl<M> FromRequestParts<M> for AdminSession
where
    M: ModulesExt,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &M,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let session = state
            .session_usecase()
            .get_from_token(Secret::new(bearer.token()))
            .map_err(|_| AuthError::InvalidToken)?
            .conv::<Self>();

        state
            .user_usecase()
            .record_activity(session.user_id)
            .await?;

        session
            .user_role
            .eq(&UserRole::Admin)
            .then_some(session)
            .ok_or(AuthError::MissingPermissions)
            .map_err(Into::into)
    }
}
