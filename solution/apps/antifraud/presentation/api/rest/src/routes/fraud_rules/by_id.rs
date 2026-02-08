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
    errors::ApiResult,
    extractors::{Json, Path, session::UserSession},
};

pub async fn get_fraud_rule_by_id<M>(
    modules: State<M>,
    UserSession {
        ..
    }: UserSession,
    Path(((), fraud_rule_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .fraud_rule_usecase()
        .get_by_id(fraud_rule_id.into())
        .await?
        .pipe(FraudRuleDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn update_fraud_rule_by_id<M>(
    modules: State<M>,
    UserSession {
        ..
    }: UserSession,
    Path(((), fraud_rule_id)): Path<((), Uuid)>,
    Json(update): Json<FraudRuleUpdateDto>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    let update_result = update.parse()?;

    modules
        .fraud_rule_usecase()
        .update_by_id(fraud_rule_id.into(), update_result)
        .await?
        .pipe(FraudRuleDto::from)
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

pub async fn disable_fraud_rule_by_id<M>(
    modules: State<M>,
    UserSession {
        ..
    }: UserSession,
    Path(((), fraud_rule_id)): Path<((), Uuid)>,
) -> ApiResult<impl IntoResponse>
where
    M: ModulesExt,
{
    modules
        .fraud_rule_usecase()
        .disable_by_id(fraud_rule_id.into())
        .await?;

    StatusCode::NO_CONTENT.pipe(Ok)
}
