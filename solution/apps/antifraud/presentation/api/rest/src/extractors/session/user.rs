use application::usecase::session::SessionUseCase as _;
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
use lib::{
    domain::Id,
    model_mapper::Mapper,
    tap::{Conv as _, Pipe as _},
};

use crate::{ApiError, ModulesExt, errors::AuthError};

#[derive(Mapper)]
#[mapper(ty = Session, from)]
pub struct UserSession {
    pub user_id: Id<User>,
    pub user_role: UserRole,
}

impl From<UserSession> for (Id<User>, UserRole) {
    fn from(session: UserSession) -> Self {
        (session.user_id, session.user_role)
    }
}

impl<M> FromRequestParts<M> for UserSession
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

        state
            .session_usecase()
            .get_from_token(dbg!(bearer).token())
            .map_err(|_| AuthError::InvalidToken)?
            .conv::<Self>()
            .pipe(Ok)
    }
}
