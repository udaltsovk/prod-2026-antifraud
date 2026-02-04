use application::usecase::fraud_rule::FraudRuleUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::{
    presentation::api::rest::{
        response::ResponseExt as _, validation::parseable::Parseable as _,
    },
    tap::Pipe as _,
    uuid::Uuid,
};

use crate::{
    ModulesExt,
    dto::fraud_rule::{FraudRuleDto, FraudRuleUpdateDto},
    errors::{ApiError, ApiResult},
    extractors::{Json, Path, session::UserSession},
};

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn get_fraud_rule_by_id<M>(
    modules: State<M>,
    UserSession {
        user_role: requester_role,
        ..
    }: UserSession,
    Path(((), user_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .fraud_rule_usecase()
        .get_by_id(requester_role, user_id.into())
        .await
        .map(FraudRuleDto::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn update_fraud_rule_by_id<M>(
    modules: State<M>,
    UserSession {
        user_role: requester_role,
        ..
    }: UserSession,
    Path(((), user_id)): Path<((), Uuid)>,
    Json(update): Json<FraudRuleUpdateDto>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let update_result = update.parse().map_err(Into::into);

    modules
        .fraud_rule_usecase()
        .update_by_id(requester_role, user_id.into(), update_result)
        .await
        .map(FraudRuleDto::from)
        .map(Json)?
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

#[cfg_attr(debug_assertions, tracing::instrument(skip(modules)))]
pub async fn disable_fraud_rule_by_id<M>(
    modules: State<M>,
    UserSession {
        user_role: requester_role,
        ..
    }: UserSession,
    Path(((), user_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .fraud_rule_usecase()
        .disable_by_id(requester_role, user_id.into())
        .await
        .map_err(ApiError::from)
        .map(|_| StatusCode::NO_CONTENT)
}
