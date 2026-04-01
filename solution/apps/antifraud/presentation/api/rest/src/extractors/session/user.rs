use application::usecase::{
    session::GetSessionFromTokenUsecase, user::RecordUserActivityUsecase,
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
use lib::{
    domain::Id,
    redact::Secret,
    tap::{Conv as _, Pipe as _},
};
use model_mapper::Mapper;

use crate::{ApiError, errors::AuthError};

#[derive(Mapper, Debug)]
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

impl<App> FromRequestParts<App> for UserSession
where
    App: Sync + GetSessionFromTokenUsecase + RecordUserActivityUsecase,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        app: &App,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let session = app
            .get_session_from_token(Secret::new(bearer.token()))
            .map_err(|_| AuthError::InvalidToken)?
            .conv::<Self>();

        app.record_user_activity(session.user_id).await?;

        session.pipe(Ok)
    }
}
